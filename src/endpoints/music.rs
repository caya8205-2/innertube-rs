use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::music::{
    MusicAlbumItem, MusicAlbumRef, MusicAlbumView, MusicArtistItem, MusicArtistPage,
    MusicArtistRef, MusicExplore, MusicHomeFeed, MusicLyrics, MusicPlaylistItem,
    MusicPlaylistView, MusicSearchFilter, MusicSearchResults, MusicShelf, MusicTrackItem,
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

/// Fetch YouTube Music playlist details and the native initial track window.
pub async fn get_music_playlist_details(
    session: &Session,
    playlist_id: &str,
) -> Result<MusicPlaylistView> {
    let clean_id = normalize_music_playlist_id(playlist_id);
    let payload = json!({
        "browseId": format!("VL{clean_id}"),
    });

    let resp = session.post_innertube_client("WEB_REMIX", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_playlist_details_response(clean_id, &raw)
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

fn normalize_music_playlist_id(playlist_id: &str) -> &str {
    let trimmed = playlist_id.trim();
    trimmed.strip_prefix("VL").unwrap_or(trimmed)
}

fn find_music_renderer<'a>(value: &'a Value, renderer_name: &str) -> Option<&'a Value> {
    match value {
        Value::Object(map) => {
            if let Some(renderer) = map.get(renderer_name) {
                return Some(renderer);
            }
            map.values()
                .find_map(|child| find_music_renderer(child, renderer_name))
        }
        Value::Array(items) => items
            .iter()
            .find_map(|child| find_music_renderer(child, renderer_name)),
        _ => None,
    }
}

fn contains_music_marker(value: &Value, marker: &str) -> bool {
    match value {
        Value::String(text) => text == marker,
        Value::Object(map) => map
            .values()
            .any(|child| contains_music_marker(child, marker)),
        Value::Array(items) => items
            .iter()
            .any(|child| contains_music_marker(child, marker)),
        _ => false,
    }
}

fn music_playlist_header(raw: &Value) -> Option<(&Value, bool, Option<String>)> {
    let header_container = raw
        .pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0")
        .or_else(|| raw.get("header"))?;

    if let Some(editable) = header_container.get("musicEditablePlaylistDetailHeaderRenderer") {
        let header = editable.pointer("/header/musicResponsiveHeaderRenderer")?;
        let privacy = editable
            .pointer("/editHeader/musicPlaylistEditHeaderRenderer/privacy")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        return Some((header, true, privacy));
    }

    let header = header_container
        .get("musicResponsiveHeaderRenderer")
        .or_else(|| header_container.get("musicDetailHeaderRenderer"))?;
    Some((header, false, Some("PUBLIC".to_string())))
}

fn music_playlist_text_runs(value: &Value) -> String {
    value
        .get("runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|run| run.get("text").and_then(Value::as_str))
                .collect::<String>()
        })
        .or_else(|| value.get("simpleText").and_then(Value::as_str).map(ToString::to_string))
        .unwrap_or_default()
}

fn music_playlist_year(header: &Value) -> Option<String> {
    header
        .pointer("/subtitle/runs")
        .and_then(Value::as_array)
        .and_then(|runs| {
            runs.iter().find_map(|run| {
                let text = run.get("text").and_then(Value::as_str)?.trim();
                (text.len() == 4 && text.chars().all(|ch| ch.is_ascii_digit()))
                    .then(|| text.to_string())
            })
        })
}

fn music_playlist_count_and_duration(header: &Value) -> (Option<u32>, Option<String>) {
    let Some(runs) = header
        .pointer("/secondSubtitle/runs")
        .and_then(Value::as_array)
    else {
        return (None, None);
    };

    let count_index = if runs.len() > 3 { 2 } else { 0 };
    let track_count = runs
        .get(count_index)
        .and_then(|run| run.get("text"))
        .and_then(Value::as_str)
        .and_then(|text| {
            let digits: String = text.chars().filter(|ch| ch.is_ascii_digit()).collect();
            (!digits.is_empty()).then_some(digits)
        })
        .and_then(|digits| digits.parse::<u32>().ok());
    let duration = runs
        .get(count_index + 2)
        .and_then(|run| run.get("text"))
        .and_then(Value::as_str)
        .map(ToString::to_string);

    (track_count, duration)
}

fn music_playlist_author(header: &Value) -> Option<MusicArtistRef> {
    let facepile = header.pointer("/facepile/avatarStackViewModel")?;
    let name = facepile
        .pointer("/text/content")
        .and_then(Value::as_str)?
        .to_string();
    let browse_id = facepile
        .pointer("/rendererContext/commandContext/onTap/innertubeCommand/browseEndpoint/browseId")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    Some(MusicArtistRef { name, browse_id })
}

/// Parse current YouTube Music playlist-detail responses.
pub fn parse_music_playlist_details_response(
    playlist_id: &str,
    raw: &Value,
) -> Result<MusicPlaylistView> {
    let clean_id = normalize_music_playlist_id(playlist_id);
    let (header, owned, privacy) = music_playlist_header(raw).ok_or_else(|| {
        InnertubeError::Other("YouTube Music playlist header was not found in response".to_string())
    })?;
    let (track_count, duration) = music_playlist_count_and_duration(header);

    let description = header
        .pointer("/description/musicDescriptionShelfRenderer")
        .and_then(MusicDescriptionShelfNode::from_value)
        .map(|shelf| shelf.description)
        .or_else(|| {
            header
                .get("description")
                .map(music_playlist_text_runs)
                .filter(|text| !text.is_empty())
        });
    let thumbnail = header
        .pointer("/thumbnail/musicThumbnailRenderer/thumbnail")
        .or_else(|| header.get("thumbnail"))
        .and_then(|value| {
            ThumbnailListNode::from_value(value)
                .best_url()
                .map(ToString::to_string)
        });

    let tracks = find_music_renderer(raw, "musicPlaylistShelfRenderer")
        .map(Parser::parse_tree)
        .map(|parsed| {
            parsed
                .find_music_items()
                .into_iter()
                .map(convert_music_node_to_track_item)
                .filter(|track| !track.video_id.is_empty() && !track.title.is_empty())
                .collect()
        })
        .unwrap_or_default();

    Ok(MusicPlaylistView {
        id: clean_id.to_string(),
        title: header
            .get("title")
            .map(music_playlist_text_runs)
            .filter(|text| !text.is_empty())
            .unwrap_or_else(|| "Untitled Playlist".to_string()),
        description,
        author: music_playlist_author(header),
        track_count,
        thumbnail,
        tracks,
        owned,
        privacy,
        duration,
        year: music_playlist_year(header),
        is_collaborative: contains_music_marker(header, "PAplaylist_collaborate"),
    })
}

/// Parse YouTube Music album page response using modular AST nodes.
pub fn parse_music_album_response(browse_id: &str, raw: &Value) -> Result<MusicAlbumView> {
    let mut album_view = MusicAlbumView {
        browse_id: browse_id.to_string(),
        ..Default::default()
    };

    // Header metadata extraction
    if let Some(header) = raw.pointer("/header/musicDetailHeaderRenderer")
        .or_else(|| raw.pointer("/header/musicResponsiveHeaderRenderer"))
        .or_else(|| raw.pointer("/header/musicVisualHeaderRenderer"))
    {
        album_view.title = header.pointer("/title/runs/0/text")
            .or_else(|| header.pointer("/title/simpleText"))
            .and_then(|t| t.as_str())
            .unwrap_or("Untitled Album")
            .to_string();

        if let Some(sub_runs) = header.pointer("/subtitle/runs").and_then(|r| r.as_array()) {
            for run in sub_runs {
                let text = run.get("text").and_then(|t| t.as_str()).unwrap_or("");
                if let Some(bid) = run.pointer("/navigationEndpoint/browseEndpoint/browseId").and_then(|b| b.as_str()) {
                    if (bid.starts_with("UC") || bid.starts_with("FEmusic_library_privately_owned_artist")) && album_view.artist.is_none() {
                        album_view.artist = Some(text.to_string());
                    }
                } else if text.chars().all(|c| c.is_ascii_digit()) && text.len() == 4 {
                    album_view.year = Some(text.to_string());
                }
            }
        }

        album_view.thumbnail = header.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url")
            .or_else(|| header.pointer("/thumbnail/thumbnails/0/url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());
    }

    // Tracklist extraction using modular parser
    let parsed_tree = Parser::parse_tree(raw);
    for item in parsed_tree.find_music_items() {
        album_view.tracks.push(convert_music_node_to_track_item(item));
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
