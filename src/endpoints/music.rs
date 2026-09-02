use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::music::{
    MusicAlbumItem, MusicAlbumRef, MusicAlbumView, MusicArtistItem, MusicArtistPage,
    MusicArtistRef, MusicExplore, MusicHomeFeed, MusicLyrics, MusicPlaylistItem,
    MusicPlaylistPage, MusicSearchFilter, MusicSearchResults, MusicShelf, MusicTrackItem,
};
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

fn normalize_music_artist_id(artist_id: &str) -> &str {
    let clean_id = artist_id.trim_start_matches("ytchannel:").trim();
    clean_id.strip_prefix("MPLA").unwrap_or(clean_id)
}

/// Fetch YouTube Music dedicated Artist Page by channel/artist ID (e.g. `UC...` or `MPLA...`).
pub async fn get_music_artist(session: &Session, artist_id: &str) -> Result<MusicArtistPage> {
    let clean_id = normalize_music_artist_id(artist_id);
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

/// Fetch a continuation page of the YouTube Music Home Feed.
pub async fn get_music_home_continuation(
    session: &Session,
    continuation_token: &str,
) -> Result<MusicHomeFeed> {
    let payload = json!({
        "continuation": continuation_token,
    });

    let resp = session.post_innertube_client("YTMUSIC", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_home_response(&raw)
}

/// Fetch the first native page of a YouTube Music playlist.
pub async fn get_music_playlist_page(
    session: &Session,
    playlist_id: &str,
) -> Result<MusicPlaylistPage> {
    let browse_id = if playlist_id.starts_with("VL") {
        playlist_id.to_string()
    } else {
        format!("VL{playlist_id}")
    };
    let resp = session
        .post_innertube_client("YTMUSIC", "/browse", json!({ "browseId": browse_id }))
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
    parse_music_playlist_response(&raw, false)
}

/// Fetch the next native page of a YouTube Music playlist.
pub async fn get_music_playlist_continuation(
    session: &Session,
    continuation_token: &str,
    is_collaborative: bool,
) -> Result<MusicPlaylistPage> {
    let resp = session
        .post_innertube_client(
            "YTMUSIC",
            "/browse",
            json!({ "continuation": continuation_token }),
        )
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
    parse_music_playlist_response(&raw, is_collaborative)
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

/// Fetch a YouTube Music playlist (`VL`-normalized, YTMUSIC client), reusing
/// the standard playlist parser.
pub async fn get_music_playlist(
    session: &Session,
    playlist_id: &str,
) -> Result<crate::models::playlist::PlaylistView> {
    let clean_id = if playlist_id.starts_with("VL") {
        playlist_id.to_string()
    } else {
        format!("VL{playlist_id}")
    };

    let resp = session
        .post_innertube_client("YTMUSIC", "/browse", json!({ "browseId": clean_id }))
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    crate::endpoints::playlist::parse_playlist_browse_response(&clean_id, &raw)
}

/// Fetch the YouTube Music library landing page (`FEmusic_library_landing`).
///
/// ponytail: legacy returns a typed `ytmusic.Library` wrapper; we return the
/// parsed node tree until typed page wrappers land.
pub async fn get_music_library(session: &Session) -> Result<Vec<YTNode>> {
    let resp = session
        .post_innertube_client(
            "YTMUSIC",
            "/browse",
            json!({ "browseId": "FEmusic_library_landing" }),
        )
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
    Ok(Parser::parse_tree(&raw))
}

/// Fetch the YouTube Music recap / listening review (`FEmusic_listening_review`).
/// Requires authentication (login-gated browseId).
pub async fn get_music_recap(session: &Session) -> Result<Vec<YTNode>> {
    let resp = session
        .post_innertube_client(
            "YTMUSIC",
            "/browse",
            json!({ "browseId": "FEmusic_listening_review" }),
        )
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
    Ok(Parser::parse_tree(&raw))
}

/// Fetch the watch-next queue panel for a track, following the automix
/// endpoint when the panel has no playlist id (legacy `Music.getUpNext`).
pub async fn get_music_up_next(
    session: &Session,
    video_id: &str,
    automix: bool,
) -> Result<crate::parser::nodes::playlist::PlaylistPanelNode> {
    let resp = session
        .post_innertube_client("YTMUSIC", "/next", json!({ "videoId": video_id }))
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let panel_value = find_playlist_panel(&raw).ok_or_else(|| {
        InnertubeError::Other(format!(
            "Music queue was empty, the given id is probably invalid. ({video_id})"
        ))
    })?;

    let panel = crate::parser::nodes::playlist::PlaylistPanelNode::from_value(panel_value)
        .ok_or_else(|| InnertubeError::Other("Could not find target tab.".to_string()))?;

    if panel.playlist_id.is_some() || !automix {
        return Ok(panel);
    }

    // Automix: follow the automix preview video's playlist endpoint.
    let automix_endpoint = find_automix_endpoint(&raw).ok_or_else(|| {
        InnertubeError::Other("Automix item not found".to_string())
    })?;

    let node = crate::parser::nodes::misc::navigation::NavigationEndpointNode::from_value(
        automix_endpoint,
    )
    .ok_or_else(|| InnertubeError::Format("Automix endpoint is not navigable".to_string()))?;
    let path = node.api_path.clone().ok_or_else(|| {
        InnertubeError::NotFound("Automix endpoint has no InnerTube API path".to_string())
    })?;

    let mut payload = node.payload.clone();
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("videoId".to_string(), json!(video_id));
    }

    let resp = session.post_innertube_client("YTMUSIC", &path, payload).await?;
    let page: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let panel_value = find_playlist_panel(&page).ok_or_else(|| {
        InnertubeError::Other("Could not fetch automix".to_string())
    })?;

    crate::parser::nodes::playlist::PlaylistPanelNode::from_value(panel_value)
        .ok_or_else(|| InnertubeError::Other("Could not fetch automix".to_string()))
}

/// Fetch the "related tracks" tab of a track (legacy `Music.getRelated`).
pub async fn get_music_related(session: &Session, video_id: &str) -> Result<Vec<YTNode>> {
    let browse_id =
        find_music_tab_browse_id(&raw_next(session, video_id).await?, "MUSIC_PAGE_TYPE_TRACK_RELATED")
            .ok_or_else(|| InnertubeError::Other("Could not find target tab.".to_string()))?;

    let resp = session
        .post_innertube_client("YTMUSIC", "/browse", json!({ "browseId": browse_id }))
        .await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
    Ok(Parser::parse_tree(&raw))
}

async fn raw_next(session: &Session, video_id: &str) -> Result<Value> {
    let resp = session
        .post_innertube_client("YTMUSIC", "/next", json!({ "videoId": video_id }))
        .await?;
    resp.json().await.map_err(InnertubeError::Network)
}

/// Find a watch-next tab's browse id by its music page type.
fn find_music_tab_browse_id(raw: &Value, page_type: &str) -> Option<String> {
    fn walk(v: &Value, page_type: &str) -> Option<String> {
        if v.get("tabRenderer").is_some() {
            let tr = &v["tabRenderer"];
            let pt = tr
                .pointer("/endpoint/browseEndpoint/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
                .and_then(Value::as_str);
            if pt == Some(page_type) {
                return tr
                    .pointer("/endpoint/browseEndpoint/browseId")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
            }
        }
        match v {
            Value::Object(map) => map.values().find_map(|x| walk(x, page_type)),
            Value::Array(items) => items.iter().find_map(|x| walk(x, page_type)),
            _ => None,
        }
    }
    walk(raw, page_type)
}

/// Locate a `playlistPanelRenderer` anywhere in the response.
fn find_playlist_panel(raw: &Value) -> Option<&Value> {
    if raw.get("playlistPanelRenderer").is_some() {
        return Some(raw);
    }
    match raw {
        Value::Object(map) => map.values().find_map(find_playlist_panel),
        Value::Array(items) => items.iter().find_map(find_playlist_panel),
        _ => None,
    }
}

/// Locate the automix preview video's playlist endpoint.
fn find_automix_endpoint(raw: &Value) -> Option<&Value> {
    if let Some(ap) = raw.get("automixPreviewVideoRenderer") {
        return ap
            .pointer("/playlistVideo/endpoint")
            .or_else(|| ap.pointer("/content/playlistVideo/endpoint"));
    }
    match raw {
        Value::Object(map) => map.values().find_map(find_automix_endpoint),
        Value::Array(items) => items.iter().find_map(find_automix_endpoint),
        _ => None,
    }
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
                    artists: item
                        .artists
                        .iter()
                        .map(|artist| MusicArtistRef {
                            name: artist.name.clone(),
                            browse_id: artist.id.clone(),
                        })
                        .collect(),
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
    crate::parser::nodes::misc::thumbnail::ThumbnailListNode::from_value(value)
        .best_url()
        .map(ToString::to_string)
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

/// Fetch YouTube Music playlist details and the native initial track window.
///
/// Distinct from `MusicManager::get_playlist` (Batch 6): this targets the
/// dedicated Music playlist page (`MusicPlaylistView` with owned/privacy/
/// duration metadata) on the YTMUSIC client, while `get_playlist` returns
/// the standard `PlaylistView` via the generic playlist parser.
pub async fn get_music_playlist_details(
    session: &Session,
    playlist_id: &str,
) -> Result<crate::models::music::MusicPlaylistView> {
    let clean_id = normalize_music_playlist_id(playlist_id);
    let payload = json!({
        "browseId": format!("VL{clean_id}"),
    });

    let resp = session.post_innertube_client("YTMUSIC", "/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_music_playlist_details_response(clean_id, &raw)
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
) -> Result<crate::models::music::MusicPlaylistView> {
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
            crate::parser::nodes::misc::thumbnail::ThumbnailListNode::from_value(value)
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

    Ok(crate::models::music::MusicPlaylistView {
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

fn music_artist_header(raw: &Value) -> Option<&Value> {
    raw.pointer("/header/musicImmersiveHeaderRenderer")
        .or_else(|| raw.pointer("/header/musicVisualHeaderRenderer"))
        .or_else(|| raw.pointer("/header/musicHeaderRenderer"))
}

fn music_artist_sections(raw: &Value) -> Option<&Vec<Value>> {
    raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents")
        .or_else(|| raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents"))
        .and_then(Value::as_array)
}

fn music_artist_shelf_title(shelf: &Value) -> &str {
    shelf
        .pointer("/title/runs/0/text")
        .or_else(|| shelf.pointer("/title/simpleText"))
        .or_else(|| shelf.pointer("/header/musicShelfHeaderRenderer/title/runs/0/text"))
        .or_else(|| shelf.pointer("/header/musicCarouselShelfBasicHeaderRenderer/title/runs/0/text"))
        .and_then(Value::as_str)
        .unwrap_or("")
}

fn music_artist_playlist_id(header: &Value, button_name: &str) -> Option<String> {
    header
        .pointer(&format!("/{button_name}/buttonRenderer/navigationEndpoint/watchEndpoint/playlistId"))
        .or_else(|| {
            header.pointer(&format!(
                "/{button_name}/buttonRenderer/navigationEndpoint/watchPlaylistEndpoint/playlistId"
            ))
        })
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn music_artist_card_year(subtitle: Option<&str>) -> Option<String> {
    subtitle?
        .split(" • ")
        .map(str::trim)
        .find(|part| part.len() == 4 && part.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToString::to_string)
}

fn find_artist_renderer<'a>(value: &'a Value, renderer_name: &str) -> Option<&'a Value> {
    match value {
        Value::Object(map) => {
            if let Some(renderer) = map.get(renderer_name) {
                return Some(renderer);
            }
            map.values()
                .find_map(|child| find_artist_renderer(child, renderer_name))
        }
        Value::Array(items) => items
            .iter()
            .find_map(|child| find_artist_renderer(child, renderer_name)),
        _ => None,
    }
}

fn music_artist_album(card: &crate::parser::nodes::music::MusicTwoRowItemNode) -> MusicAlbumItem {
    MusicAlbumItem {
        browse_id: card.id.clone().unwrap_or_default(),
        title: card.title.clone(),
        artist: None,
        artists: Vec::new(),
        year: music_artist_card_year(card.subtitle.as_deref()),
        thumbnail: card.thumbnails.best_url().map(ToString::to_string),
        track_count: None,
    }
}

fn music_artist_video(card: &crate::parser::nodes::music::MusicTwoRowItemNode) -> MusicTrackItem {
    MusicTrackItem {
        video_id: card.id.clone().unwrap_or_default(),
        title: card.title.clone(),
        thumbnail: card.thumbnails.best_url().map(ToString::to_string),
        ..Default::default()
    }
}

/// Parse YouTube Music Artist Page response (`Artist.ts`).
pub fn parse_music_artist_response(artist_id: &str, raw: &Value) -> Result<MusicArtistPage> {
    let mut page = MusicArtistPage {
        id: normalize_music_artist_id(artist_id).to_string(),
        ..Default::default()
    };

    if let Some(header) = music_artist_header(raw) {
        page.name = header
            .pointer("/title/runs/0/text")
            .or_else(|| header.pointer("/title/simpleText"))
            .and_then(Value::as_str)
            .unwrap_or("Unknown Artist")
            .to_string();

        page.description = header
            .pointer("/description/runs/0/text")
            .or_else(|| header.pointer("/description/simpleText"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let subscription = header.pointer("/subscriptionButton/subscribeButtonRenderer");
        page.channel_id = subscription
            .and_then(|button| button.get("channelId"))
            .and_then(Value::as_str)
            .map(ToString::to_string);
        page.subscribers = subscription
            .and_then(|button| button.get("subscriberCountText"))
            .and_then(|text| {
                text.pointer("/runs/0/text")
                    .or_else(|| text.get("simpleText"))
            })
            .and_then(Value::as_str)
            .map(ToString::to_string);
        page.subscribed = subscription
            .and_then(|button| button.get("subscribed"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        page.monthly_listeners = header
            .pointer("/monthlyListenerCount/runs/0/text")
            .and_then(Value::as_str)
            .map(|text| text.replace(" monthly audience", ""));
        page.shuffle_id = music_artist_playlist_id(header, "playButton");
        page.radio_id = music_artist_playlist_id(header, "startRadioButton");

        let thumbnail = header
            .pointer("/thumbnail/musicThumbnailRenderer/thumbnail")
            .or_else(|| header.pointer("/foregroundThumbnail/musicThumbnailRenderer/thumbnail"));
        page.thumbnail = thumbnail
            .and_then(|value| crate::parser::nodes::misc::thumbnail::ThumbnailListNode::from_value(value).best_url().map(ToString::to_string));
    }

    if let Some(description) = find_artist_renderer(raw, "musicDescriptionShelfRenderer") {
        if let Some(shelf) = MusicDescriptionShelfNode::from_value(description) {
            page.description = Some(shelf.description);
        }
        page.views = description
            .pointer("/subheader/runs/0/text")
            .and_then(Value::as_str)
            .map(ToString::to_string);
    }

    if let Some(sections) = music_artist_sections(raw) {
        for section in sections {
            let shelf = section
                .get("musicShelfRenderer")
                .or_else(|| section.get("musicCarouselShelfRenderer"))
                .unwrap_or(section);
            let title = music_artist_shelf_title(shelf);
            let parsed_shelf = Parser::parse_tree(shelf);

            if title.eq_ignore_ascii_case("Songs") || title.eq_ignore_ascii_case("Top songs") {
                page.top_songs.extend(
                    parsed_shelf
                        .find_music_items()
                        .into_iter()
                        .map(convert_music_node_to_track_item)
                        .filter(|track| !track.video_id.is_empty()),
                );
            } else if title.eq_ignore_ascii_case("Albums") {
                page.albums.extend(parsed_shelf.iter().filter_map(|node| match node {
                    YTNode::MusicCard(card) if !card.title.is_empty() => Some(music_artist_album(card)),
                    _ => None,
                }));
            } else if title.eq_ignore_ascii_case("Singles")
                || title.eq_ignore_ascii_case("Singles & EPs")
            {
                page.singles.extend(parsed_shelf.iter().filter_map(|node| match node {
                    YTNode::MusicCard(card) if !card.title.is_empty() => Some(music_artist_album(card)),
                    _ => None,
                }));
            } else if title.eq_ignore_ascii_case("Videos") {
                page.videos.extend(parsed_shelf.iter().filter_map(|node| match node {
                    YTNode::MusicCard(card) if !card.title.is_empty() => {
                        let video = music_artist_video(card);
                        (!video.video_id.is_empty()).then_some(video)
                    }
                    YTNode::MusicItem(item) => {
                        let video = convert_music_node_to_track_item(item);
                        (!video.video_id.is_empty()).then_some(video)
                    }
                    _ => None,
                }));
            } else if title.eq_ignore_ascii_case("Fans might also like") || title.eq_ignore_ascii_case("Similar artists") {
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

fn find_music_playlist_container(value: &Value) -> Option<&Value> {
    if let Some(container) = value.pointer("/continuationContents/musicPlaylistShelfContinuation") {
        return Some(container);
    }
    if let Some(actions) = value.get("onResponseReceivedActions").and_then(Value::as_array) {
        for action in actions {
            if let Some(container) = action.get("appendContinuationItemsAction") {
                return Some(container);
            }
        }
    }

    match value {
        Value::Object(map) => {
            if let Some(container) = map.get("musicPlaylistShelfRenderer") {
                return Some(container);
            }
            map.values().find_map(find_music_playlist_container)
        }
        Value::Array(items) => items.iter().find_map(find_music_playlist_container),
        _ => None,
    }
}

fn music_playlist_items(container: &Value) -> Option<&Vec<Value>> {
    container
        .get("contents")
        .or_else(|| container.get("continuationItems"))
        .and_then(Value::as_array)
}

fn music_playlist_continuation_token(container: &Value, items: &[Value]) -> Option<String> {
    let direct = items.iter().find_map(|item| {
        item.pointer("/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
            .or_else(|| {
                item.pointer(
                    "/continuationItemViewModel/continuationEndpoint/continuationCommand/token",
                )
            })
            .and_then(Value::as_str)
            .map(ToString::to_string)
    });
    direct.or_else(|| {
        container
            .pointer("/continuations/0/nextContinuationData/continuation")
            .or_else(|| container.pointer("/continuations/0/reloadContinuationData/continuation"))
            .and_then(Value::as_str)
            .map(ToString::to_string)
    })
}

fn contains_playlist_collaboration_marker(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            if map.get("tag").and_then(Value::as_str) == Some("PAplaylist_collaborate") {
                return true;
            }
            map.values().any(contains_playlist_collaboration_marker)
        }
        Value::Array(items) => items.iter().any(contains_playlist_collaboration_marker),
        _ => false,
    }
}

/// Parse one native YouTube Music playlist page.
pub fn parse_music_playlist_response(
    raw: &Value,
    is_collaborative_hint: bool,
) -> Result<MusicPlaylistPage> {
    let container = find_music_playlist_container(raw).ok_or_else(|| {
        InnertubeError::Other("YouTube Music playlist shelf was not found in response".to_string())
    })?;
    let items = music_playlist_items(container).ok_or_else(|| {
        InnertubeError::Other("YouTube Music playlist page did not contain items".to_string())
    })?;

    let tracks = items
        .iter()
        .filter_map(MusicResponsiveListItemNode::from_value)
        .map(|item| convert_music_node_to_track_item(&item))
        .filter(|track| !track.video_id.is_empty() && !track.title.is_empty())
        .collect();
    let continuation_token = music_playlist_continuation_token(container, items);
    let is_collaborative = is_collaborative_hint || contains_playlist_collaboration_marker(raw);

    Ok(MusicPlaylistPage {
        tracks,
        continuation_token,
        is_collaborative,
    })
}

/// Parse YouTube Music Home Feed response (`HomeFeed.ts`).
pub fn parse_music_home_response(raw: &Value) -> Result<MusicHomeFeed> {
    let mut feed = MusicHomeFeed::default();

    if let Some(sections) = raw
        .pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents")
        .or_else(|| raw.pointer("/continuationContents/sectionListContinuation/contents"))
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
                            artists: Vec::new(),
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
                            track_count: card.track_count,
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

    feed.continuation_token = Parser::parse_tree(raw)
        .find_continuation_token()
        .or_else(|| {
            raw.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/continuations/0/nextContinuationData/continuation")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        });
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
                artists: Vec::new(),
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
        like_status: crate::models::music::MusicLikeStatus::from_api_status(
            item.like_status.as_deref(),
        ),
    }
}
