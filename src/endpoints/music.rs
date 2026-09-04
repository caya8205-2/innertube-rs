use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::music::{
    MusicAlbumItem, MusicAlbumRef, MusicAlbumView, MusicArtistItem, MusicArtistPage,
    MusicArtistRef, MusicExplore, MusicHomeFeed, MusicLyrics, MusicPlaylistItem,
    MusicSearchFilter, MusicSearchResults, MusicShelf, MusicTrackItem,
};
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;
use crate::parser::nodes::music::{MusicDescriptionShelfNode, MusicResponsiveListItemNode};
use crate::parser::{NodeListExt, Parser, YTNode};

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

    let resp = session.post_innertube_client("YTMUSIC", "/search", payload).await?;
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

    let next_resp = session.post_innertube_client("YTMUSIC", "/next", next_payload).await?;
    let next_raw: Value = next_resp.json().await.map_err(InnertubeError::Network)?;

    let lyrics_browse_id = extract_lyrics_browse_id(&next_raw).ok_or_else(|| {
        InnertubeError::Other(format!("No lyrics available on YouTube Music for video: {}", video_id))
    })?;

    // Step 2: Call /browse with the lyrics browseId
    let browse_payload = json!({
        "browseId": lyrics_browse_id
    });

    let browse_resp = session.post_innertube_client("YTMUSIC", "/browse", browse_payload).await?;
    let browse_raw: Value = browse_resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_lyrics_response(&browse_raw)
}

/// Fetch details and full tracklist of a YouTube Music album by browse ID (e.g. `MPREb_...`).
pub async fn get_music_album(session: &Session, browse_id: &str) -> Result<MusicAlbumView> {
    let payload = json!({
        "browseId": browse_id
    });

    let resp = session.post_innertube_client("YTMUSIC", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_album_response(browse_id, &raw)
}

/// Fetch YouTube Music dedicated Artist Page by channel/artist ID (e.g. `UC...`).
pub async fn get_music_artist(session: &Session, artist_id: &str) -> Result<MusicArtistPage> {
    let clean_id = artist_id.trim_start_matches("ytchannel:").trim();
    let payload = json!({
        "browseId": clean_id,
    });

    let resp = session.post_innertube_client("YTMUSIC", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_artist_response(clean_id, &raw)
}

/// Fetch YouTube Music Home Feed (`FEmusic_home`).
pub async fn get_music_home(session: &Session) -> Result<MusicHomeFeed> {
    let payload = json!({
        "browseId": "FEmusic_home",
    });

    let resp = session.post_innertube_client("YTMUSIC", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_home_response(&raw)
}

/// Fetch YouTube Music Explore page (New Releases, Charts, Moods & Genres).
pub async fn get_music_explore(session: &Session) -> Result<MusicExplore> {
    let payload = json!({
        "browseId": "FEmusic_explore"
    });

    let resp = session.post_innertube_client("YTMUSIC", "/browse", payload).await?;
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

/// Parse YouTube Music search results using modular AST nodes.
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

    let parsed_tree = Parser::parse_tree(raw);

    for item in parsed_tree.find_music_items() {
        let track = convert_music_node_to_track_item(item);

        match filter {
            Some(MusicSearchFilter::Songs) => results.songs.push(track),
            Some(MusicSearchFilter::Videos) => results.videos.push(track),
            Some(MusicSearchFilter::Albums) => {
                results.albums.push(MusicAlbumItem {
                    browse_id: item.album_id.clone().or_else(|| item.id.clone()).unwrap_or_default(),
                    title: item.title.clone(),
                    artist: item.artists.first().map(|a| a.name.clone()),
                    year: None,
                    thumbnail: item.thumbnails.best_url().map(|s| s.to_string()),
                    track_count: None,
                });
            }
            Some(MusicSearchFilter::Artists) => {
                results.artists.push(MusicArtistItem {
                    browse_id: item.id.clone().unwrap_or_default(),
                    name: item.title.clone(),
                    subscribers: None,
                    thumbnail: item.thumbnails.best_url().map(|s| s.to_string()),
                });
            }
            Some(MusicSearchFilter::Playlists) => {
                results.playlists.push(MusicPlaylistItem {
                    browse_id: item.id.clone().unwrap_or_default(),
                    title: item.title.clone(),
                    author: item.artists.first().map(|a| a.name.clone()),
                    track_count: None,
                    thumbnail: item.thumbnails.best_url().map(|s| s.to_string()),
                });
            }
            None => {
                results.songs.push(track);
            }
            _ => {
                results.playlists.push(MusicPlaylistItem {
                    browse_id: item.id.clone().unwrap_or_default(),
                    title: item.title.clone(),
                    author: item.artists.first().map(|a| a.name.clone()),
                    track_count: None,
                    thumbnail: item.thumbnails.best_url().map(|s| s.to_string()),
                });
            }
        }
    }

    Ok(results)
}

/// Parse YouTube Music lyrics response using modular AST nodes.
pub fn parse_music_lyrics_response(raw: &Value) -> Result<MusicLyrics> {
    let parsed_tree = Parser::parse_tree(raw);

    for node in &parsed_tree {
        if let YTNode::MusicDescriptionShelf(shelf) = node {
            return Ok(MusicLyrics {
                lyrics_text: shelf.description.clone(),
                footer: shelf.footer.clone(),
                title: shelf.header.clone(),
                is_synced: false,
            });
        }
    }

    // Direct fallback from header / renderer
    if let Some(shelf) = raw.pointer("/contents/sectionListRenderer/contents/0/musicDescriptionShelfRenderer") {
        if let Some(desc) = MusicDescriptionShelfNode::from_value(shelf) {
            return Ok(MusicLyrics {
                lyrics_text: desc.description,
                footer: desc.footer,
                title: desc.header,
                is_synced: false,
            });
        }
    }

    Err(InnertubeError::Other("Lyrics text shelf not found in response".to_string()))
}

fn music_album_header(raw: &Value) -> Option<&Value> {
    raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0/musicResponsiveHeaderRenderer")
        .or_else(|| raw.pointer("/header/musicDetailHeaderRenderer"))
        .or_else(|| raw.pointer("/header/musicResponsiveHeaderRenderer"))
        .or_else(|| raw.pointer("/header/musicVisualHeaderRenderer"))
}

fn music_text(value: &Value) -> Option<String> {
    if let Some(text) = value.get("simpleText").and_then(Value::as_str) {
        return Some(text.to_string());
    }
    let text = value
        .get("runs")?
        .as_array()?
        .iter()
        .filter_map(|run| run.get("text").and_then(Value::as_str))
        .collect::<String>();
    (!text.is_empty()).then_some(text)
}

fn music_album_artists(header: &Value) -> Vec<MusicArtistRef> {
    let runs = header
        .pointer("/straplineTextOne/runs")
        .or_else(|| header.pointer("/subtitle/runs"))
        .and_then(Value::as_array);
    runs.into_iter()
        .flatten()
        .filter_map(|run| {
            let browse_id = run
                .pointer("/navigationEndpoint/browseEndpoint/browseId")
                .and_then(Value::as_str)?;
            if !browse_id.starts_with("UC")
                && !browse_id.starts_with("FEmusic_library_privately_owned_artist")
            {
                return None;
            }
            let name = run.get("text").and_then(Value::as_str)?.trim();
            (!name.is_empty()).then(|| MusicArtistRef {
                name: name.to_string(),
                browse_id: Some(browse_id.to_string()),
            })
        })
        .collect()
}

fn music_album_audio_playlist_id(header: &Value) -> Option<String> {
    if let Some(playlist_id) = header
        .get("buttons")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find_map(|button| {
            button
                .pointer("/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/playlistId")
                .or_else(|| button.pointer("/musicPlayButtonRenderer/playNavigationEndpoint/watchPlaylistEndpoint/playlistId"))
                .and_then(Value::as_str)
        })
    {
        return Some(playlist_id.to_string());
    }

    header
        .pointer("/menu/menuRenderer/topLevelButtons")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find_map(|button| {
            button
                .pointer("/buttonRenderer/navigationEndpoint/watchEndpoint/playlistId")
                .or_else(|| button.pointer("/buttonRenderer/navigationEndpoint/watchPlaylistEndpoint/playlistId"))
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
}

fn music_album_thumbnail(header: &Value) -> Option<String> {
    let value = header
        .pointer("/thumbnail/musicThumbnailRenderer/thumbnail")
        .or_else(|| header.pointer("/thumbnail/croppedSquareThumbnailRenderer/thumbnail"))
        .or_else(|| header.get("thumbnail"))?;
    ThumbnailListNode::from_value(value).best_url().map(ToString::to_string)
}

fn parse_music_count(text: &str) -> Option<u32> {
    let digits: String = text.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

fn value_contains_string(value: &Value, needle: &str) -> bool {
    match value {
        Value::String(text) => text == needle,
        Value::Array(items) => items.iter().any(|item| value_contains_string(item, needle)),
        Value::Object(map) => map.values().any(|item| value_contains_string(item, needle)),
        _ => false,
    }
}

/// Parse YouTube Music album page response, including the current responsive-header layout.
pub fn parse_music_album_response(browse_id: &str, raw: &Value) -> Result<MusicAlbumView> {
    let mut album_view = MusicAlbumView {
        browse_id: browse_id.to_string(),
        ..Default::default()
    };

    if let Some(header) = music_album_header(raw) {
        album_view.title = header
            .get("title")
            .and_then(music_text)
            .unwrap_or_else(|| "Untitled Album".to_string());

        if let Some(subtitle_runs) = header.pointer("/subtitle/runs").and_then(Value::as_array) {
            album_view.album_type = subtitle_runs
                .first()
                .and_then(|run| run.get("text"))
                .and_then(Value::as_str)
                .map(ToString::to_string);
            album_view.year = subtitle_runs.iter().find_map(|run| {
                let text = run.get("text").and_then(Value::as_str)?;
                (text.len() == 4 && text.chars().all(|ch| ch.is_ascii_digit()))
                    .then(|| text.to_string())
            });
        }

        album_view.artists = music_album_artists(header);
        album_view.artist = (!album_view.artists.is_empty()).then(|| {
            album_view
                .artists
                .iter()
                .map(|artist| artist.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        });

        if let Some(second_runs) = header.pointer("/secondSubtitle/runs").and_then(Value::as_array) {
            let texts: Vec<&str> = second_runs
                .iter()
                .filter_map(|run| run.get("text").and_then(Value::as_str))
                .filter(|text| *text != " • ")
                .collect();
            album_view.track_count = texts.first().and_then(|text| parse_music_count(text));
            album_view.duration = texts.get(1).map(|text| (*text).to_string()).or_else(|| {
                (texts.len() == 1 && album_view.track_count.is_none())
                    .then(|| texts[0].to_string())
            });
        }

        album_view.description = header
            .pointer("/description/musicDescriptionShelfRenderer/description")
            .or_else(|| header.get("description"))
            .and_then(music_text);
        album_view.thumbnail = music_album_thumbnail(header);
        album_view.audio_playlist_id = music_album_audio_playlist_id(header);
        album_view.is_explicit = header
            .get("subtitleBadge")
            .or_else(|| header.get("subtitleBadges"))
            .is_some_and(|badge| {
                value_contains_string(badge, "MUSIC_EXPLICIT_BADGE")
                    || value_contains_string(badge, "Explicit")
            });
    }

    let parsed_tree = Parser::parse_tree(raw);
    for item in parsed_tree.find_music_items() {
        let track = convert_music_node_to_track_item(item);
        if !track.video_id.is_empty() {
            album_view.tracks.push(track);
        }
    }

    if album_view.track_count.is_none() && !album_view.tracks.is_empty() {
        album_view.track_count = u32::try_from(album_view.tracks.len()).ok();
    }
    let complete_tracklist = album_view
        .track_count
        .map(|count| count as usize == album_view.tracks.len())
        .unwrap_or(true);
    if complete_tracklist
        && !album_view.tracks.is_empty()
        && album_view.tracks.iter().all(|track| track.duration_ms.is_some())
    {
        album_view.duration_ms = Some(album_view.tracks.iter().filter_map(|track| track.duration_ms).sum());
    }

    Ok(album_view)
}

/// Parse YouTube Music Artist Page response (`Artist.ts`).
pub fn parse_music_artist_response(artist_id: &str, raw: &Value) -> Result<MusicArtistPage> {
    let mut page = MusicArtistPage {
        id: artist_id.to_string(),
        ..Default::default()
    };

    // Header extraction
    if let Some(header) = raw.pointer("/header/musicImmersiveHeaderRenderer")
        .or_else(|| raw.pointer("/header/musicVisualHeaderRenderer"))
        .or_else(|| raw.pointer("/header/musicHeaderRenderer"))
    {
        page.name = header.pointer("/title/runs/0/text")
            .or_else(|| header.pointer("/title/simpleText"))
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown Artist")
            .to_string();

        page.description = header.pointer("/description/runs/0/text")
            .or_else(|| header.pointer("/description/simpleText"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        page.subscribers = header.pointer("/subscriptionButton/subscribeButtonRenderer/subscriberCountText/runs/0/text")
            .or_else(|| header.pointer("/subscriptionButton/subscribeButtonRenderer/subscriberCountText/simpleText"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        page.thumbnail = header.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url")
            .or_else(|| header.pointer("/foregroundThumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());
    }

    // Extract Shelves from sectionListRenderer
    if let Some(sections) = raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents")
        .and_then(|s| s.as_array())
    {
        for sec in sections {
            let shelf_target = sec.get("musicShelfRenderer")
                .or_else(|| sec.get("musicCarouselShelfRenderer"))
                .unwrap_or(sec);

            let shelf_title = shelf_target.pointer("/header/musicShelfHeaderRenderer/title/runs/0/text")
                .or_else(|| shelf_target.pointer("/header/musicCarouselShelfBasicHeaderRenderer/title/runs/0/text"))
                .and_then(|t| t.as_str())
                .unwrap_or("");

            let parsed_shelf = Parser::parse_tree(shelf_target);

            if shelf_title.eq_ignore_ascii_case("Songs") || shelf_title.eq_ignore_ascii_case("Top songs") {
                for item in parsed_shelf.find_music_items() {
                    page.top_songs.push(convert_music_node_to_track_item(item));
                }
            } else if shelf_title.eq_ignore_ascii_case("Albums") {
                for node in &parsed_shelf {
                    if let YTNode::MusicCard(card) = node {
                        page.albums.push(MusicAlbumItem {
                            browse_id: card.id.clone().unwrap_or_default(),
                            title: card.title.clone(),
                            artist: card.subtitle.clone(),
                            year: None,
                            thumbnail: card.thumbnails.best_url().map(|s| s.to_string()),
                            track_count: None,
                        });
                    }
                }
            } else if shelf_title.eq_ignore_ascii_case("Singles") || shelf_title.eq_ignore_ascii_case("Singles & EPs") {
                for node in &parsed_shelf {
                    if let YTNode::MusicCard(card) = node {
                        page.singles.push(MusicAlbumItem {
                            browse_id: card.id.clone().unwrap_or_default(),
                            title: card.title.clone(),
                            artist: card.subtitle.clone(),
                            year: None,
                            thumbnail: card.thumbnails.best_url().map(|s| s.to_string()),
                            track_count: None,
                        });
                    }
                }
            } else if shelf_title.eq_ignore_ascii_case("Videos") {
                for item in parsed_shelf.find_music_items() {
                    page.videos.push(convert_music_node_to_track_item(item));
                }
            } else if shelf_title.eq_ignore_ascii_case("Fans might also like") || shelf_title.eq_ignore_ascii_case("Similar artists") {
                for node in &parsed_shelf {
                    if let YTNode::MusicCard(card) = node {
                        page.similar_artists.push(MusicArtistItem {
                            browse_id: card.id.clone().unwrap_or_default(),
                            name: card.title.clone(),
                            subscribers: card.subtitle.clone(),
                            thumbnail: card.thumbnails.best_url().map(|s| s.to_string()),
                        });
                    }
                }
            }
        }
    }

    Ok(page)
}

/// Parse YouTube Music Home Feed response (`HomeFeed.ts`).
pub fn parse_music_home_response(raw: &Value) -> Result<MusicHomeFeed> {
    let mut feed = MusicHomeFeed::default();

    if let Some(sections) = raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents")
        .and_then(|s| s.as_array())
    {
        for sec in sections {
            let shelf_target = sec.get("musicCarouselShelfRenderer")
                .or_else(|| sec.get("musicShelfRenderer"))
                .unwrap_or(sec);

            let title = shelf_target.pointer("/header/musicCarouselShelfBasicHeaderRenderer/title/runs/0/text")
                .or_else(|| shelf_target.pointer("/header/musicShelfHeaderRenderer/title/runs/0/text"))
                .and_then(|t| t.as_str())
                .unwrap_or("Featured")
                .to_string();

            let subtitle = shelf_target.pointer("/header/musicCarouselShelfBasicHeaderRenderer/strapline/runs/0/text")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());

            let mut shelf = MusicShelf {
                title,
                subtitle,
                tracks: Vec::new(),
                albums: Vec::new(),
                playlists: Vec::new(),
            };

            let parsed_shelf = Parser::parse_tree(shelf_target);
            for item in parsed_shelf.find_music_items() {
                shelf.tracks.push(convert_music_node_to_track_item(item));
            }
            for node in &parsed_shelf {
                if let YTNode::MusicCard(card) = node {
                    if card.item_type.as_deref() == Some("MUSIC_PAGE_TYPE_ALBUM") {
                        shelf.albums.push(MusicAlbumItem {
                            browse_id: card.id.clone().unwrap_or_default(),
                            title: card.title.clone(),
                            artist: card.subtitle.clone(),
                            year: None,
                            thumbnail: card.thumbnails.best_url().map(|s| s.to_string()),
                            track_count: None,
                        });
                    } else {
                        shelf.playlists.push(MusicPlaylistItem {
                            browse_id: card.id.clone().unwrap_or_default(),
                            title: card.title.clone(),
                            author: card.subtitle.clone(),
                            track_count: None,
                            thumbnail: card.thumbnails.best_url().map(|s| s.to_string()),
                        });
                    }
                }
            }

            if !shelf.tracks.is_empty() || !shelf.albums.is_empty() || !shelf.playlists.is_empty() {
                feed.shelves.push(shelf);
            }
        }
    }

    feed.continuation_token = Parser::parse_tree(raw).find_continuation_token();
    Ok(feed)
}

/// Parse YouTube Music explore / charts response using modular AST nodes.
pub fn parse_music_explore_response(raw: &Value) -> Result<MusicExplore> {
    let mut explore = MusicExplore::default();
    let parsed_tree = Parser::parse_tree(raw);

    // Extract trending tracks
    for item in parsed_tree.find_music_items() {
        explore.top_songs.push(convert_music_node_to_track_item(item));
    }

    // Extract albums & categories from two-row items
    for node in &parsed_tree {
        if let YTNode::MusicCard(card) = node {
            explore.new_releases.push(MusicAlbumItem {
                browse_id: card.id.clone().unwrap_or_default(),
                title: card.title.clone(),
                artist: card.subtitle.clone(),
                year: None,
                thumbnail: card.thumbnails.best_url().map(|s| s.to_string()),
                track_count: None,
            });
        }
    }

    Ok(explore)
}

fn convert_music_node_to_track_item(item: &MusicResponsiveListItemNode) -> MusicTrackItem {
    MusicTrackItem {
        video_id: item.id.clone().unwrap_or_default(),
        title: item.title.clone(),
        artists: item.artists.iter().map(|a| MusicArtistRef {
            name: a.name.clone(),
            browse_id: a.id.clone(),
        }).collect(),
        album: item.album.as_ref().map(|title| MusicAlbumRef {
            title: title.clone(),
            browse_id: item.album_id.clone(),
        }),
        duration: item.duration.clone(),
        duration_ms: item.duration_ms,
        thumbnail: item.thumbnails.best_url().map(|s| s.to_string()),
        is_explicit: item.is_explicit,
    }
}
