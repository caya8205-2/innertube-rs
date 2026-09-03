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
            .and_then(|value| ThumbnailListNode::from_value(value).best_url().map(ToString::to_string));
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
            } else if title.eq_ignore_ascii_case("Fans might also like")
                || title.eq_ignore_ascii_case("Similar artists")
            {
                page.similar_artists.extend(parsed_shelf.iter().filter_map(|node| match node {
                    YTNode::MusicCard(card) if !card.title.is_empty() => Some(MusicArtistItem {
                        browse_id: card.id.clone().unwrap_or_default(),
                        name: card.title.clone(),
                        subscribers: card.subtitle.clone(),
                        thumbnail: card.thumbnails.best_url().map(ToString::to_string),
                    }),
                    _ => None,
                }));
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
