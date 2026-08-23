use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::music::{
    MusicAlbumItem, MusicAlbumRef, MusicAlbumView, MusicArtistItem, MusicArtistRef,
    MusicExplore, MusicLyrics, MusicPlaylistItem, MusicSearchFilter, MusicSearchResults,
    MusicTrackItem,
};

/// Perform a filtered search on YouTube Music (`WEB_REMIX`).
pub async fn search_music(
    session: &Session,
    query: &str,
    filter: Option<MusicSearchFilter>,
) -> Result<MusicSearchResults> {
    let mut payload = json!({
        "query": query,
    });

    if let Some(f) = filter {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("params".to_string(), json!(f.to_param_str()));
        }
    }

    let resp = session.post_innertube_client("WEB_REMIX", "/search", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_search_response(query, filter, &raw)
}

/// Fetch song lyrics from YouTube Music for a given video ID.
pub async fn get_music_lyrics(session: &Session, video_id: &str) -> Result<MusicLyrics> {
    // Step 1: Call /next with WEB_REMIX to obtain the lyrics tab browseId
    let next_payload = json!({
        "videoId": video_id,
        "isAudioOnly": true
    });

    let next_resp = session.post_innertube_client("WEB_REMIX", "/next", next_payload).await?;
    let next_raw: Value = next_resp.json().await.map_err(InnertubeError::Network)?;

    let lyrics_browse_id = extract_lyrics_browse_id(&next_raw).ok_or_else(|| {
        InnertubeError::Other(format!("No lyrics available on YouTube Music for video: {}", video_id))
    })?;

    // Step 2: Call /browse with the lyrics browseId
    let browse_payload = json!({
        "browseId": lyrics_browse_id
    });

    let browse_resp = session.post_innertube_client("WEB_REMIX", "/browse", browse_payload).await?;
    let browse_raw: Value = browse_resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_lyrics_response(&browse_raw)
}

/// Fetch details and full tracklist of a YouTube Music album by browse ID (e.g. `MPREb_...`).
pub async fn get_music_album(session: &Session, browse_id: &str) -> Result<MusicAlbumView> {
    let payload = json!({
        "browseId": browse_id
    });

    let resp = session.post_innertube_client("WEB_REMIX", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_album_response(browse_id, &raw)
}

/// Fetch YouTube Music Explore page (New Releases, Charts, Moods & Genres).
pub async fn get_music_explore(session: &Session) -> Result<MusicExplore> {
    let payload = json!({
        "browseId": "FEmusic_explore"
    });

    let resp = session.post_innertube_client("WEB_REMIX", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_explore_response(&raw)
}

// ---------------------------------------------------------------------------
// Response Parsers
// ---------------------------------------------------------------------------

fn extract_lyrics_browse_id(raw: &Value) -> Option<String> {
    let tabs = raw.pointer("/contents/singleColumnMusicWatchNextResultsRenderer/tabbedRenderer/watchNextTabbedResultsRenderer/tabs")
        .or_else(|| raw.pointer("/contents/tabbedSearchResultsRenderer/tabs"))
        .and_then(|t| t.as_array())?;

    for tab in tabs {
        let tr = tab.get("tabRenderer")?;
        let title = tr.get("title").and_then(|t| t.as_str()).unwrap_or("");
        let page_type = tr.pointer("/endpoint/browseEndpoint/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
            .and_then(|p| p.as_str())
            .unwrap_or("");

        if title.eq_ignore_ascii_case("Lyrics") || page_type == "MUSIC_PAGE_TYPE_TRACK_LYRICS" {
            if let Some(browse_id) = tr.pointer("/endpoint/browseEndpoint/browseId").and_then(|b| b.as_str()) {
                return Some(browse_id.to_string());
            }
        }
    }

    None
}

/// Parse lyrics browse response into `MusicLyrics`.
pub fn parse_music_lyrics_response(raw: &Value) -> Result<MusicLyrics> {
    if let Some(msg_renderer) = raw.pointer("/contents/messageRenderer") {
        let msg = parse_runs_text(msg_renderer.get("text")).unwrap_or_else(|| "Lyrics not available".to_string());
        return Err(InnertubeError::Other(msg));
    }

    let shelf = raw.pointer("/contents/sectionListRenderer/contents/0/musicDescriptionShelfRenderer")
        .or_else(|| raw.pointer("/contents/musicDescriptionShelfRenderer"))
        .ok_or_else(|| InnertubeError::Other("Lyrics not available for this track".into()))?;

    let title = shelf.pointer("/header/runs/0/text")
        .or_else(|| shelf.pointer("/title/runs/0/text"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let lyrics_text = parse_runs_text(shelf.get("description")).unwrap_or_default();
    let footer = parse_runs_text(shelf.get("footer"));

    Ok(MusicLyrics {
        lyrics_text,
        footer,
        title,
        is_synced: false,
    })
}

/// Parse YouTube Music search response into `MusicSearchResults`.
pub fn parse_music_search_response(
    query: &str,
    filter: Option<MusicSearchFilter>,
    raw: &Value,
) -> Result<MusicSearchResults> {
    let mut results = MusicSearchResults {
        query: query.to_string(),
        filter,
        ..Default::default()
    };

    let section_contents = raw.pointer("/contents/tabbedSearchResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents")
        .or_else(|| raw.pointer("/contents/sectionListRenderer/contents"))
        .and_then(|c| c.as_array());

    let sections = match section_contents {
        Some(s) => s,
        None => return Ok(results),
    };

    for sec in sections {
        let mut items_to_process = Vec::new();
        let mut shelf_title = String::new();

        if let Some(shelf) = sec.get("musicShelfRenderer") {
            shelf_title = shelf.pointer("/title/runs/0/text").and_then(|t| t.as_str()).unwrap_or("").to_lowercase();
            if let Some(items) = shelf.get("contents").and_then(|c| c.as_array()) {
                items_to_process.extend(items.iter());
            }
            if let Some(token) = shelf.pointer("/continuations/0/nextContinuationData/continuation").and_then(|t| t.as_str()) {
                results.continuation_token = Some(token.to_string());
            }
        } else if let Some(isr) = sec.get("itemSectionRenderer") {
            if let Some(contents) = isr.get("contents").and_then(|c| c.as_array()) {
                for c in contents {
                    if let Some(shelf) = c.get("musicShelfRenderer") {
                        shelf_title = shelf.pointer("/title/runs/0/text").and_then(|t| t.as_str()).unwrap_or("").to_lowercase();
                        if let Some(items) = shelf.get("contents").and_then(|c| c.as_array()) {
                            items_to_process.extend(items.iter());
                        }
                    } else if c.get("musicResponsiveListItemRenderer").is_some() {
                        items_to_process.push(c);
                    }
                }
            }
        } else if let Some(card) = sec.get("musicCardShelfRenderer") {
            if let Some(contents) = card.get("contents").and_then(|c| c.as_array()) {
                items_to_process.extend(contents.iter());
            }
        }

        for item in items_to_process {
            if let Some(mrli) = item.get("musicResponsiveListItemRenderer") {
                let (track, album, artist, playlist) = parse_music_responsive_item(mrli);

                if let Some(t) = track {
                    if shelf_title.contains("video") {
                        results.videos.push(t);
                    } else {
                        results.songs.push(t);
                    }
                }
                if let Some(al) = album {
                    results.albums.push(al);
                }
                if let Some(ar) = artist {
                    results.artists.push(ar);
                }
                if let Some(pl) = playlist {
                    results.playlists.push(pl);
                }
            }
        }
    }

    Ok(results)
}

fn parse_music_responsive_item(
    item: &Value,
) -> (
    Option<MusicTrackItem>,
    Option<MusicAlbumItem>,
    Option<MusicArtistItem>,
    Option<MusicPlaylistItem>,
) {
    let mut title = String::new();
    let mut video_id = None;
    let mut browse_id = None;
    let mut is_explicit = false;

    // Check badges
    if let Some(badges) = item.get("badges").and_then(|b| b.as_array()) {
        for b in badges {
            if let Some(label) = b.pointer("/musicInlineBadgeRenderer/accessibilityData/accessibilityData/label").and_then(|l| l.as_str()) {
                if label.eq_ignore_ascii_case("Explicit") {
                    is_explicit = true;
                }
            }
        }
    }

    // Extract title & primary navigation
    if let Some(col0) = item.pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text") {
        if let Some(runs) = col0.get("runs").and_then(|r| r.as_array()) {
            if let Some(first) = runs.first() {
                title = first.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string();
                if video_id.is_none() {
                    video_id = first.pointer("/navigationEndpoint/watchEndpoint/videoId").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
                if browse_id.is_none() {
                    browse_id = first.pointer("/navigationEndpoint/browseEndpoint/browseId").and_then(|b| b.as_str()).map(|s| s.to_string());
                }
            }
        }
    }

    if video_id.is_none() {
        video_id = item.pointer("/navigationEndpoint/watchEndpoint/videoId")
            .or_else(|| item.pointer("/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId"))
            .or_else(|| item.pointer("/doubleTapCommand/watchEndpoint/videoId"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }

    if browse_id.is_none() {
        browse_id = item.pointer("/navigationEndpoint/browseEndpoint/browseId")
            .or_else(|| item.pointer("/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/browseEndpoint/browseId"))
            .and_then(|b| b.as_str())
            .map(|s| s.to_string());
    }

    let root_page_type = item.pointer("/navigationEndpoint/browseEndpoint/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
        .and_then(|p| p.as_str())
        .unwrap_or("");

    // Extract thumbnail
    let thumbnail = item.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url")
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    // Extract artists, album, duration from column 1
    let mut artists = Vec::new();
    let mut album_ref = None;
    let mut duration = None;
    let mut item_type = "song";

    if let Some(col1) = item.pointer("/flexColumns/1/musicResponsiveListItemFlexColumnRenderer/text/runs").and_then(|r| r.as_array()) {
        for run in col1 {
            let text = run.get("text").and_then(|t| t.as_str()).unwrap_or("").trim();
            if text == "•" || text.is_empty() {
                continue;
            }

            if text.eq_ignore_ascii_case("Song") {
                item_type = "song";
                continue;
            } else if text.eq_ignore_ascii_case("Video") {
                item_type = "video";
                continue;
            } else if text.eq_ignore_ascii_case("Album") || text.eq_ignore_ascii_case("EP") || text.eq_ignore_ascii_case("Single") {
                item_type = "album";
                continue;
            } else if text.eq_ignore_ascii_case("Artist") {
                item_type = "artist";
                continue;
            } else if text.eq_ignore_ascii_case("Playlist") {
                item_type = "playlist";
                continue;
            }

            let nav_browse_id = run.pointer("/navigationEndpoint/browseEndpoint/browseId").and_then(|b| b.as_str());
            let page_type = run.pointer("/navigationEndpoint/browseEndpoint/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
                .and_then(|p| p.as_str())
                .unwrap_or("");

            if page_type == "MUSIC_PAGE_TYPE_ARTIST" || (nav_browse_id.is_some() && nav_browse_id.unwrap().starts_with("UC")) {
                artists.push(MusicArtistRef {
                    name: text.to_string(),
                    browse_id: nav_browse_id.map(|s| s.to_string()),
                });
            } else if page_type == "MUSIC_PAGE_TYPE_ALBUM" || (nav_browse_id.is_some() && nav_browse_id.unwrap().starts_with("MPRE")) {
                album_ref = Some(MusicAlbumRef {
                    title: text.to_string(),
                    browse_id: nav_browse_id.map(|s| s.to_string()),
                });
            } else if text.contains(':') && text.chars().all(|c| c.is_ascii_digit() || c == ':') {
                duration = Some(text.to_string());
            } else if artists.is_empty() && !text.chars().all(|c| c.is_ascii_digit()) {
                artists.push(MusicArtistRef {
                    name: text.to_string(),
                    browse_id: nav_browse_id.map(|s| s.to_string()),
                });
            }
        }
    }

    if let Some(vid) = video_id {
        let duration_ms = duration.as_deref().and_then(parse_duration_to_ms);
        return (
            Some(MusicTrackItem {
                video_id: vid,
                title,
                artists,
                album: album_ref,
                duration,
                duration_ms,
                thumbnail,
                is_explicit,
            }),
            None,
            None,
            None,
        );
    }

    if let Some(bid) = browse_id {
        if bid.starts_with("MPRE") || root_page_type == "MUSIC_PAGE_TYPE_ALBUM" || item_type == "album" {
            let artist_name = artists.first().map(|a| a.name.clone());
            return (
                None,
                Some(MusicAlbumItem {
                    browse_id: bid,
                    title,
                    artist: artist_name,
                    year: None,
                    thumbnail,
                    track_count: None,
                }),
                None,
                None,
            );
        } else if bid.starts_with("UC") || root_page_type == "MUSIC_PAGE_TYPE_ARTIST" || item_type == "artist" {
            return (
                None,
                None,
                Some(MusicArtistItem {
                    browse_id: bid,
                    name: title,
                    subscribers: None,
                    thumbnail,
                }),
                None,
            );
        } else if bid.starts_with("VL") || bid.starts_with("PL") || bid.starts_with("RDCLAK") || root_page_type == "MUSIC_PAGE_TYPE_PLAYLIST" || item_type == "playlist" {
            return (
                None,
                None,
                None,
                Some(MusicPlaylistItem {
                    browse_id: bid,
                    title,
                    author: artists.first().map(|a| a.name.clone()),
                    track_count: None,
                    thumbnail,
                }),
            );
        }
    }

    (None, None, None, None)
}

/// Parse YouTube Music album browse response into `MusicAlbumView`.
pub fn parse_music_album_response(browse_id: &str, raw: &Value) -> Result<MusicAlbumView> {
    let header = raw.pointer("/header/musicDetailHeaderRenderer")
        .or_else(|| raw.pointer("/header/musicResponsiveHeaderRenderer"))
        .or_else(|| raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0/musicResponsiveHeaderRenderer"))
        .or_else(|| raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0/musicResponsiveHeaderRenderer"))
        .or_else(|| raw.pointer("/contents/twoColumnBrowseResultsRenderer/secondaryContents/sectionListRenderer/contents/0/musicResponsiveHeaderRenderer"));

    let title = header
        .and_then(|h| h.pointer("/title/runs/0/text").or_else(|| h.pointer("/title/simpleText")))
        .and_then(|t| t.as_str())
        .unwrap_or("Unknown Album")
        .to_string();

    let artist = header
        .and_then(|h| h.pointer("/straplineTextOne/runs/0/text").or_else(|| h.pointer("/subtitle/runs/0/text")))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let year = header
        .and_then(|h| h.pointer("/subtitle/runs/2/text").or_else(|| h.pointer("/subtitle/runs/4/text")))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let description = header
        .and_then(|h| h.pointer("/description/runs/0/text"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let thumbnail = header
        .and_then(|h| {
            h.pointer("/thumbnail/croppedSquareThumbnailRenderer/thumbnail/thumbnails/0/url")
                .or_else(|| h.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url"))
        })
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    let mut tracks = Vec::new();

    // Check all section sources for musicShelfRenderer
    let mut all_sections = Vec::new();

    if let Some(sec) = raw.pointer("/contents/twoColumnBrowseResultsRenderer/secondaryContents/sectionListRenderer/contents").and_then(|c| c.as_array()) {
        all_sections.extend(sec.iter());
    }
    if let Some(sec) = raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents").and_then(|c| c.as_array()) {
        all_sections.extend(sec.iter());
    }
    if let Some(sec) = raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents").and_then(|c| c.as_array()) {
        all_sections.extend(sec.iter());
    }
    if let Some(sec) = raw.pointer("/contents/sectionListRenderer/contents").and_then(|c| c.as_array()) {
        all_sections.extend(sec.iter());
    }

    for sec in all_sections {
        if let Some(shelf) = sec.get("musicShelfRenderer") {
            if let Some(items) = shelf.get("contents").and_then(|c| c.as_array()) {
                for item in items {
                    if let Some(mrli) = item.get("musicResponsiveListItemRenderer") {
                        let (track, _, _, _) = parse_music_responsive_item(mrli);
                        if let Some(t) = track {
                            tracks.push(t);
                        }
                    }
                }
            }
        }
    }

    Ok(MusicAlbumView {
        browse_id: browse_id.to_string(),
        title,
        artist,
        year,
        description,
        thumbnail,
        tracks,
    })
}

/// Parse YouTube Music explore page into `MusicExplore`.
pub fn parse_music_explore_response(raw: &Value) -> Result<MusicExplore> {
    let mut explore = MusicExplore::default();

    let sections = raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents")
        .or_else(|| raw.pointer("/contents/sectionListRenderer/contents"))
        .and_then(|c| c.as_array());

    if let Some(sec_list) = sections {
        for sec in sec_list {
            if let Some(carousel) = sec.get("musicCarouselShelfRenderer") {
                let title = carousel.pointer("/header/musicCarouselShelfBasicHeaderRenderer/title/runs/0/text")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_lowercase();

                if let Some(items) = carousel.get("contents").and_then(|c| c.as_array()) {
                    for item in items {
                        if let Some(mrli) = item.get("musicResponsiveListItemRenderer") {
                            let (track, album, artist, _) = parse_music_responsive_item(mrli);
                            if let Some(t) = track {
                                if title.contains("video") {
                                    explore.top_videos.push(t);
                                } else {
                                    explore.top_songs.push(t);
                                }
                            }
                            if let Some(al) = album {
                                explore.new_releases.push(al);
                            }
                            if let Some(ar) = artist {
                                explore.top_artists.push(ar);
                            }
                        } else if let Some(mttm) = item.get("musicTwoRowItemRenderer") {
                            let item_title = mttm.pointer("/title/runs/0/text").and_then(|t| t.as_str()).unwrap_or("").to_string();
                            let browse_id = mttm.pointer("/navigationEndpoint/browseEndpoint/browseId").and_then(|b| b.as_str()).unwrap_or("").to_string();
                            let thumbnail = mttm.pointer("/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails/0/url").and_then(|u| u.as_str()).map(|s| s.to_string());

                            if !browse_id.is_empty() {
                                explore.new_releases.push(MusicAlbumItem {
                                    browse_id,
                                    title: item_title,
                                    artist: None,
                                    year: None,
                                    thumbnail,
                                    track_count: None,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(explore)
}

fn parse_runs_text(val: Option<&Value>) -> Option<String> {
    let val = val?;
    if let Some(s) = val.get("simpleText").and_then(|s| s.as_str()) {
        return Some(s.to_string());
    }
    if let Some(runs) = val.get("runs").and_then(|r| r.as_array()) {
        let texts: Vec<&str> = runs.iter().filter_map(|r| r.get("text").and_then(|t| t.as_str())).collect();
        if !texts.is_empty() {
            return Some(texts.join(""));
        }
    }
    None
}

fn parse_duration_to_ms(text: &str) -> Option<u64> {
    let parts: Vec<&str> = text.split(':').collect();
    match parts.len() {
        2 => {
            let mins = parts[0].parse::<u64>().ok()?;
            let secs = parts[1].parse::<u64>().ok()?;
            Some((mins * 60 + secs) * 1000)
        }
        3 => {
            let hrs = parts[0].parse::<u64>().ok()?;
            let mins = parts[1].parse::<u64>().ok()?;
            let secs = parts[2].parse::<u64>().ok()?;
            Some((hrs * 3600 + mins * 60 + secs) * 1000)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_music_lyrics() {
        let fixture = json!({
            "contents": {
                "sectionListRenderer": {
                    "contents": [
                        {
                            "musicDescriptionShelfRenderer": {
                                "header": {
                                    "runs": [{ "text": "Lyrics" }]
                                },
                                "description": {
                                    "runs": [
                                        { "text": "We're no strangers to love\n" },
                                        { "text": "You know the rules and so do I" }
                                    ]
                                },
                                "footer": {
                                    "runs": [{ "text": "Source: LyricFind" }]
                                }
                            }
                        }
                    ]
                }
            }
        });

        let lyrics = parse_music_lyrics_response(&fixture).expect("Failed to parse lyrics");
        assert_eq!(lyrics.title.as_deref(), Some("Lyrics"));
        assert!(lyrics.lyrics_text.contains("We're no strangers to love"));
        assert_eq!(lyrics.footer.as_deref(), Some("Source: LyricFind"));
    }

    #[test]
    fn test_parse_music_search() {
        let fixture = json!({
            "contents": {
                "tabbedSearchResultsRenderer": {
                    "tabs": [
                        {
                            "tabRenderer": {
                                "content": {
                                    "sectionListRenderer": {
                                        "contents": [
                                            {
                                                "musicShelfRenderer": {
                                                    "title": { "runs": [{ "text": "Songs" }] },
                                                    "contents": [
                                                        {
                                                            "musicResponsiveListItemRenderer": {
                                                                "flexColumns": [
                                                                    {
                                                                        "musicResponsiveListItemFlexColumnRenderer": {
                                                                            "text": {
                                                                                "runs": [
                                                                                    {
                                                                                        "text": "Never Gonna Give You Up",
                                                                                        "navigationEndpoint": {
                                                                                            "watchEndpoint": { "videoId": "dQw4w9WgXcQ" }
                                                                                        }
                                                                                    }
                                                                                ]
                                                                            }
                                                                        }
                                                                    },
                                                                    {
                                                                        "musicResponsiveListItemFlexColumnRenderer": {
                                                                            "text": {
                                                                                "runs": [
                                                                                    {
                                                                                        "text": "Rick Astley",
                                                                                        "navigationEndpoint": {
                                                                                            "browseEndpoint": {
                                                                                                "browseId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                                                                                                "browseEndpointContextSupportedConfigs": {
                                                                                                    "browseEndpointContextMusicConfig": {
                                                                                                        "pageType": "MUSIC_PAGE_TYPE_ARTIST"
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    },
                                                                                    { "text": " • " },
                                                                                    { "text": "3:32" }
                                                                                ]
                                                                            }
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            }
                        }
                    ]
                }
            }
        });

        let results = parse_music_search_response("rick astley", Some(MusicSearchFilter::Songs), &fixture).expect("Failed to parse music search");
        assert_eq!(results.songs.len(), 1);
        assert_eq!(results.songs[0].video_id, "dQw4w9WgXcQ");
        assert_eq!(results.songs[0].title, "Never Gonna Give You Up");
        assert_eq!(results.songs[0].artists[0].name, "Rick Astley");
        assert_eq!(results.songs[0].duration.as_deref(), Some("3:32"));
        assert_eq!(results.songs[0].duration_ms, Some(212000));
    }
}
