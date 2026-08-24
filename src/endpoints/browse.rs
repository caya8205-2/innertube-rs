use serde_json::Value;
use std::collections::HashSet;
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::channel::{ChannelArtistView, ChannelPlaylist, ChannelTrack, YouTubePlaylistView};
use crate::parser::nodes::channel::ChannelHeaderNode;
use crate::parser::{NodeListExt, Parser, YTNode};

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
                if let Some(header) = ChannelHeaderNode::from_value(&json_data) {
                    if !header.title.is_empty() && header.title != "Unknown Channel" {
                        channel_name = header.title;
                    }
                    avatar = header.avatar.best_url().map(|s| s.to_string());
                    subscribers = header.subscriber_count;
                }
            }

            let parsed_tree = Parser::parse_tree(&json_data);
            for v in parsed_tree.find_videos() {
                if seen_vids.insert(v.id.clone()) {
                    let thumb = v.thumbnails.best_url().unwrap_or("").to_string();
                    videos.push((v.title.clone(), v.id.clone(), thumb));
                }
            }
            for p in parsed_tree.find_playlists() {
                if !p.id.is_empty() && seen_pls.insert(p.id.clone()) {
                    let thumb = p.thumbnails.best_url().unwrap_or("").to_string();
                    playlists.push((p.title.clone(), p.id.clone(), thumb));
                }
            }
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

/// Fetch playlist details from standard YouTube playlist page (`/playlist?list=...`).
pub async fn get_youtube_playlist(
    session: &Session,
    playlist_id: &str,
) -> Result<YouTubePlaylistView> {
    let clean_id = playlist_id.trim_start_matches("playlist:").trim();
    let url = format!("https://www.youtube.com/playlist?list={clean_id}");

    let resp = session.http_client.get(&url).send().await.map_err(InnertubeError::Network)?;
    let html = resp.text().await.map_err(InnertubeError::Network)?;

    let json_data = extract_yt_initial_data(&html).ok_or_else(|| {
        InnertubeError::Other(format!("Failed to extract ytInitialData from playlist: {}", clean_id))
    })?;

    let parsed_tree = Parser::parse_tree(&json_data);
    let mut tracks = Vec::new();
    let mut seen_ids = HashSet::new();

    let mut playlist_name = "YouTube Playlist".to_string();
    let mut playlist_image = None;

    if let Some(p) = parsed_tree.find_playlists().first() {
        if !p.title.is_empty() {
            playlist_name = p.title.clone();
        }
        playlist_image = p.thumbnails.best_url().map(|s| s.to_string());
    }

    for node in &parsed_tree {
        match node {
            YTNode::PlaylistVideo(pv) => {
                if seen_ids.insert(pv.id.clone()) {
                    tracks.push(ChannelTrack {
                        id: format!("youtube:{}", pv.id),
                        title: pv.title.clone(),
                        artist: pv.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                        artist_id: pv.author.as_ref().and_then(|a| a.id.clone()).unwrap_or_default(),
                        album: playlist_name.clone(),
                        duration: pv.duration_ms.map(|d| (d / 1000) as u32).unwrap_or(180),
                        thumbnail: pv.thumbnails.best_url().unwrap_or("").to_string(),
                        youtube_id: pv.id.clone(),
                    });
                }
            }
            YTNode::Video(v) => {
                if seen_ids.insert(v.id.clone()) {
                    tracks.push(ChannelTrack {
                        id: format!("youtube:{}", v.id),
                        title: v.title.clone(),
                        artist: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                        artist_id: v.author.as_ref().and_then(|a| a.id.clone()).unwrap_or_default(),
                        album: playlist_name.clone(),
                        duration: v.duration_ms.map(|d| (d / 1000) as u32).unwrap_or(180),
                        thumbnail: v.thumbnails.best_url().unwrap_or("").to_string(),
                        youtube_id: v.id.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(YouTubePlaylistView {
        id: clean_id.to_string(),
        name: playlist_name,
        image: playlist_image,
        tracks,
    })
}

fn extract_yt_initial_data(html: &str) -> Option<Value> {
    let marker = "var ytInitialData = ";
    let alt_marker = "window[\"ytInitialData\"] = ";

    let (start_idx, skip_len) = if let Some(idx) = html.find(marker) {
        (idx, marker.len())
    } else {
        let idx = html.find(alt_marker)?;
        (idx, alt_marker.len())
    };

    let start = start_idx + skip_len;
    let remainder = &html[start..];

    let end = remainder.find(";</script>").or_else(|| remainder.find(";\n"))?;
    let json_str = &remainder[..end];

    serde_json::from_str(json_str).ok()
}
