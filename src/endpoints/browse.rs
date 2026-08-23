use serde_json::{json, Value};
use std::collections::HashSet;
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::channel::{ChannelArtistView, ChannelPlaylist, ChannelTrack, YouTubePlaylistView};

/// Fetch channel data (avatar, subscriber count, top tracks, and playlists).
pub async fn get_channel(
    session: &Session,
    channel_id_or_handle: &str,
) -> Result<ChannelArtistView> {
    let clean_id = channel_id_or_handle
        .trim_start_matches("ytchannel:")
        .trim_start_matches("youtube:")
        .trim_start_matches("channel:")
        .trim();

    let is_uc = clean_id.starts_with("UC");
    let is_handle = clean_id.starts_with('@');

    let base_path = if is_uc {
        format!("channel/{clean_id}")
    } else if is_handle {
        clean_id.to_string()
    } else {
        format!("@{clean_id}")
    };

    let home_url = format!("https://www.youtube.com/{base_path}");
    let videos_url = format!("https://www.youtube.com/{base_path}/videos");
    let releases_url = format!("https://www.youtube.com/{base_path}/releases");

    // Fetch HTML pages concurrently
    let (home_res, videos_res, releases_res) = tokio::join!(
        session.http_client.get(&home_url).send(),
        session.http_client.get(&videos_url).send(),
        session.http_client.get(&releases_url).send()
    );

    let home_html = home_res.map_err(InnertubeError::Network)?.text().await.unwrap_or_default();
    let videos_html = videos_res.map_err(InnertubeError::Network)?.text().await.unwrap_or_default();
    let releases_html = releases_res.map_err(InnertubeError::Network)?.text().await.unwrap_or_default();

    let mut videos: Vec<(String, String, String)> = Vec::new();
    let mut playlists: Vec<(String, String, String)> = Vec::new();
    let mut seen_vids: HashSet<String> = HashSet::new();
    let mut seen_pls: HashSet<String> = HashSet::new();
    let mut channel_name = String::new();
    let mut avatar = None;
    let mut subscribers = None;

    for html in [&home_html, &videos_html, &releases_html] {
        if let Some(json_data) = extract_yt_initial_data(html) {
            if channel_name.is_empty() {
                if let Some(meta_name) = json_data.pointer("/metadata/channelMetadataRenderer/title").and_then(Value::as_str) {
                    channel_name = meta_name.to_string();
                }
                if avatar.is_none() {
                    if let Some(thumb) = json_data.pointer("/metadata/channelMetadataRenderer/avatar/thumbnails/0/url").and_then(Value::as_str) {
                        avatar = Some(clean_url(thumb));
                    }
                }
                if subscribers.is_none() {
                    if let Some(subs) = json_data.pointer("/header/pageHeaderRenderer/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/0/text/content").and_then(Value::as_str) {
                        subscribers = Some(subs.to_string());
                    }
                }
            }

            parse_channel_node(&json_data, &mut videos, &mut playlists, &mut seen_vids, &mut seen_pls);
        }
    }

    if channel_name.is_empty() {
        channel_name = clean_id.to_string();
    }

    let top_tracks: Vec<ChannelTrack> = videos
        .into_iter()
        .map(|(title, video_id, thumbnail)| ChannelTrack {
            id: format!("youtube:{video_id}"),
            title,
            artist: channel_name.clone(),
            artist_id: format!("ytchannel:{clean_id}"),
            album: channel_name.clone(),
            duration: 180,
            thumbnail,
            youtube_id: video_id,
        })
        .collect();

    let channel_playlists: Vec<ChannelPlaylist> = playlists
        .into_iter()
        .map(|(name, id, image)| ChannelPlaylist {
            url: format!("https://www.youtube.com/playlist?list={id}"),
            id,
            name,
            total_tracks: 0,
            image: if image.is_empty() { None } else { Some(image) },
        })
        .collect();

    Ok(ChannelArtistView {
        id: format!("ytchannel:{clean_id}"),
        name: channel_name,
        genres: Vec::new(),
        popularity: None,
        followers: subscribers,
        image: avatar,
        spotify_url: None,
        top_tracks,
        albums: Vec::new(),
        channel_playlists,
    })
}

/// Fetch playlist tracklist using `/youtubei/v1/browse` with YouTube Music context.
pub async fn get_playlist(
    session: &Session,
    playlist_id: &str,
) -> Result<YouTubePlaylistView> {
    let clean_pid = playlist_id
        .trim_start_matches("ytplaylist:")
        .trim_start_matches("youtube:")
        .trim();

    let browse_id = if clean_pid.starts_with("VL") {
        clean_pid.to_string()
    } else {
        format!("VL{clean_pid}")
    };

    let payload = json!({
        "context": {
            "client": {
                "clientName": "WEB_REMIX",
                "clientVersion": "1.20250219.01.00",
                "hl": "en",
                "gl": "US"
            }
        },
        "browseId": browse_id
    });

    let resp = session.post_innertube("/browse", payload).await?;

    if !resp.status().is_success() {
        return Err(InnertubeError::Api {
            status: resp.status().to_string(),
            message: format!("Browse endpoint returned HTTP {}", resp.status()),
        });
    }

    let json_val: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let playlist_name = json_val
        .pointer("/header/musicResponsiveHeaderRenderer/title/runs/0/text")
        .or_else(|| json_val.pointer("/header/musicHeaderRenderer/title/runs/0/text"))
        .or_else(|| json_val.pointer("/metadata/playlistMetadataRenderer/title"))
        .or_else(|| json_val.pointer("/header/playlistHeaderRenderer/title/simpleText"))
        .and_then(Value::as_str)
        .unwrap_or("Unknown Playlist")
        .to_string();

    let playlist_image = json_val
        .pointer("/header/musicResponsiveHeaderRenderer/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url")
        .or_else(|| json_val.pointer("/header/playlistHeaderRenderer/playlistHeaderBanner/heroPlaylistThumbnailRenderer/thumbnail/thumbnails/0/url"))
        .and_then(Value::as_str)
        .map(clean_url);

    let mut tracks = Vec::new();
    let mut seen_ids = HashSet::new();

    parse_playlist_tracks(&json_val, &mut tracks, &mut seen_ids);

    Ok(YouTubePlaylistView {
        id: clean_pid.to_string(),
        name: playlist_name,
        image: playlist_image,
        tracks,
    })
}

fn extract_yt_initial_data(html: &str) -> Option<Value> {
    let patterns = ["var ytInitialData = ", "window[\"ytInitialData\"] = "];
    for pat in patterns {
        if let Some(start_idx) = html.find(pat) {
            let json_part = &html[start_idx + pat.len()..];
            let end_idx = json_part
                .find(";</script>")
                .or_else(|| json_part.find(";\n"))
                .or_else(|| json_part.find(";var "))
                .or_else(|| json_part.find(";window"))
                .unwrap_or(json_part.len());

            let clean_json = json_part[..end_idx].trim().trim_end_matches(';');
            if let Ok(val) = serde_json::from_str::<Value>(clean_json) {
                return Some(val);
            }
        }
    }
    None
}

fn clean_url(url_str: &str) -> String {
    let u = url_str.trim();
    if u.starts_with("//") {
        format!("https:{u}")
    } else {
        u.to_string()
    }
}

fn parse_channel_node(
    value: &Value,
    videos: &mut Vec<(String, String, String)>,
    playlists: &mut Vec<(String, String, String)>,
    seen_vids: &mut HashSet<String>,
    seen_pls: &mut HashSet<String>,
) {
    if let Some(arr) = value.as_array() {
        for v in arr {
            parse_channel_node(v, videos, playlists, seen_vids, seen_pls);
        }
    } else if let Some(obj) = value.as_object() {
        if let Some(lvm) = obj.get("lockupViewModel") {
            let content_id = lvm.get("contentId").and_then(Value::as_str).unwrap_or("");
            let title = lvm.pointer("/metadata/lockupMetadataViewModel/title/content").and_then(Value::as_str).unwrap_or("");

            if content_id.len() == 11 && !title.is_empty() && !seen_vids.contains(content_id) {
                let thumb = lvm.pointer("/contentImage/thumbnailViewModel/image/sources/0/url").and_then(Value::as_str).unwrap_or("");
                seen_vids.insert(content_id.to_string());
                videos.push((title.to_string(), content_id.to_string(), clean_url(thumb)));
            } else if !content_id.is_empty() && content_id.len() != 11 && !title.is_empty() {
                let clean_p_id = content_id.trim_start_matches("VL").to_string();
                if !seen_pls.contains(&clean_p_id) {
                    let thumb = lvm.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/image/sources/0/url").and_then(Value::as_str).unwrap_or("");
                    seen_pls.insert(clean_p_id.clone());
                    playlists.push((title.to_string(), clean_p_id, clean_url(thumb)));
                }
            }
        }

        if let Some(vr) = obj.get("videoRenderer") {
            let v_id = vr.get("videoId").and_then(Value::as_str).unwrap_or("");
            let title = vr.pointer("/title/runs/0/text").and_then(Value::as_str).unwrap_or("");
            let thumb = vr.pointer("/thumbnail/thumbnails/0/url").and_then(Value::as_str).unwrap_or("");
            if !v_id.is_empty() && v_id.len() == 11 && !title.is_empty() && !seen_vids.contains(v_id) {
                seen_vids.insert(v_id.to_string());
                videos.push((title.to_string(), v_id.to_string(), clean_url(thumb)));
            }
        }

        for (_, v) in obj {
            parse_channel_node(v, videos, playlists, seen_vids, seen_pls);
        }
    }
}

fn parse_playlist_tracks(
    value: &Value,
    tracks: &mut Vec<ChannelTrack>,
    seen_ids: &mut HashSet<String>,
) {
    if let Some(arr) = value.as_array() {
        for v in arr {
            parse_playlist_tracks(v, tracks, seen_ids);
        }
    } else if let Some(obj) = value.as_object() {
        if let Some(item) = obj.get("musicResponsiveListItemRenderer") {
            let video_id = item
                .pointer("/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId")
                .or_else(|| item.pointer("/playlistItemData/videoId"))
                .and_then(Value::as_str)
                .unwrap_or("");

            let title = item
                .pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/text")
                .and_then(Value::as_str)
                .unwrap_or("Unknown Track");

            let artist = item
                .pointer("/flexColumns/1/musicResponsiveListItemFlexColumnRenderer/text/runs/0/text")
                .and_then(Value::as_str)
                .unwrap_or("Unknown Artist");

            let thumbnail = item
                .pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails/0/url")
                .and_then(Value::as_str)
                .map(clean_url)
                .unwrap_or_default();

            if !video_id.is_empty() && !seen_ids.contains(video_id) {
                seen_ids.insert(video_id.to_string());
                tracks.push(ChannelTrack {
                    id: format!("youtube:{video_id}"),
                    title: title.to_string(),
                    artist: artist.to_string(),
                    artist_id: "ytplaylist".to_string(),
                    album: String::new(),
                    duration: 180,
                    thumbnail,
                    youtube_id: video_id.to_string(),
                });
            }
        }

        if let Some(item) = obj.get("playlistVideoRenderer").or_else(|| obj.get("playlistPanelVideoRenderer")) {
            let video_id = item.get("videoId").and_then(Value::as_str).unwrap_or("");
            let title = item.pointer("/title/runs/0/text")
                .or_else(|| item.pointer("/title/simpleText"))
                .and_then(Value::as_str)
                .unwrap_or("Unknown Track");

            let artist = item.pointer("/shortBylineText/runs/0/text")
                .and_then(Value::as_str)
                .unwrap_or("Unknown Artist");

            let thumbnail = item.pointer("/thumbnail/thumbnails/0/url")
                .and_then(Value::as_str)
                .map(clean_url)
                .unwrap_or_default();

            if !video_id.is_empty() && !seen_ids.contains(video_id) {
                seen_ids.insert(video_id.to_string());
                tracks.push(ChannelTrack {
                    id: format!("youtube:{video_id}"),
                    title: title.to_string(),
                    artist: artist.to_string(),
                    artist_id: "ytplaylist".to_string(),
                    album: String::new(),
                    duration: 180,
                    thumbnail,
                    youtube_id: video_id.to_string(),
                });
            }
        }

        for (_, v) in obj {
            parse_playlist_tracks(v, tracks, seen_ids);
        }
    }
}
