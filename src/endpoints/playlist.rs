use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::Result;
use crate::models::playlist::{PlaylistContinuation, PlaylistVideoItem, PlaylistView};

/// Fetch full YouTube playlist metadata and video list.
pub async fn get_playlist(session: &Session, playlist_id: &str) -> Result<PlaylistView> {
    let clean_id = if playlist_id.starts_with("VL") {
        playlist_id.to_string()
    } else {
        format!("VL{}", playlist_id)
    };

    let payload = json!({
        "browseId": clean_id,
    });

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_playlist_browse_response(&clean_id, &raw)
}

/// Fetch next page of playlist videos using a continuation token.
pub async fn get_playlist_continuation(
    session: &Session,
    continuation_token: &str,
) -> Result<PlaylistContinuation> {
    let payload = json!({
        "continuation": continuation_token,
    });

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_playlist_continuation_response(&raw)
}

/// Parse playlist browse response into `PlaylistView`.
pub fn parse_playlist_browse_response(playlist_id: &str, raw: &Value) -> Result<PlaylistView> {
    let mut view = PlaylistView {
        id: playlist_id.to_string(),
        ..Default::default()
    };

    // 1. Extract Header metadata
    if let Some(header) = raw.pointer("/header/playlistHeaderRenderer").or_else(|| raw.pointer("/header/pageHeaderRenderer")) {
        view.title = header.pointer("/title/runs/0/text")
            .or_else(|| header.pointer("/title/simpleText"))
            .or_else(|| header.pointer("/pageTitle"))
            .or_else(|| header.pointer("/content/pageHeaderViewModel/title/dynamicTextViewModel/text/content"))
            .and_then(|t| t.as_str())
            .unwrap_or("Untitled Playlist")
            .to_string();

        view.author = header.pointer("/ownerText/runs/0/text")
            .or_else(|| header.pointer("/author/runs/0/text"))
            .or_else(|| header.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/avatarStack/avatarStackViewModel/text/content"))
            .and_then(|t| t.as_str())
            .map(|s| s.strip_prefix("by ").unwrap_or(s).to_string());

        view.author_id = header.pointer("/ownerText/runs/0/navigationEndpoint/browseEndpoint/browseId")
            .or_else(|| header.pointer("/author/runs/0/navigationEndpoint/browseEndpoint/browseId"))
            .or_else(|| header.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/avatarStack/avatarStackViewModel/text/commandRuns/0/onTap/innertubeCommand/browseEndpoint/browseId"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        view.description = header.pointer("/descriptionText/runs/0/text")
            .or_else(|| header.pointer("/descriptionText/simpleText"))
            .or_else(|| header.pointer("/content/pageHeaderViewModel/description/descriptionViewModel/description/content"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        view.thumbnail = header.pointer("/playlistHeaderBanner/heroPlaylistThumbnailRenderer/thumbnail/thumbnails/0/url")
            .or_else(|| header.pointer("/image/thumbnails/0/url"))
            .or_else(|| header.pointer("/content/pageHeaderViewModel/heroContent/previewImage/contentPreviewImageViewModel/image/sources/0/url"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        if let Some(num_str) = header.pointer("/numVideosText/runs/0/text")
            .or_else(|| header.pointer("/numVideosText/simpleText"))
            .or_else(|| header.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/1/text/content"))
            .and_then(|t| t.as_str())
        {
            let digits: String = num_str.chars().filter(|c| c.is_ascii_digit()).collect();
            view.video_count = digits.parse().ok();
        }
    }

    // 2. Extract Videos & Continuation Token
    if let Some(tabs) = raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs").and_then(|t| t.as_array()) {
        for tab in tabs {
            if let Some(sections) = tab.pointer("/tabRenderer/content/sectionListRenderer/contents").and_then(|s| s.as_array()) {
                for sec in sections {
                    if let Some(items) = sec.pointer("/itemSectionRenderer/contents/0/playlistVideoListRenderer/contents").and_then(|i| i.as_array()) {
                        for item in items {
                            if let Some(video) = parse_playlist_item_value(item) {
                                view.videos.push(video);
                            } else if let Some(cir) = item.get("continuationItemRenderer") {
                                view.continuation_token = cir.pointer("/continuationEndpoint/continuationCommand/token")
                                    .and_then(|t| t.as_str())
                                    .map(|s| s.to_string());
                            }
                        }
                    } else if let Some(items) = sec.pointer("/itemSectionRenderer/contents").and_then(|i| i.as_array()) {
                        for item in items {
                            if let Some(video) = parse_playlist_item_value(item) {
                                view.videos.push(video);
                            }
                        }
                    } else if let Some(cir) = sec.get("continuationItemViewModel").or_else(|| sec.get("continuationItemRenderer")) {
                        view.continuation_token = cir.pointer("/continuationEndpoint/continuationCommand/token")
                            .or_else(|| cir.pointer("/continuationEndpoint/command/token"))
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string());
                    }
                }
            }
        }
    }

    Ok(view)
}

/// Parse continuation response into `PlaylistContinuation`.
pub fn parse_playlist_continuation_response(raw: &Value) -> Result<PlaylistContinuation> {
    let mut result = PlaylistContinuation::default();

    let items = raw.pointer("/onResponseReceivedActions/0/appendContinuationItemsAction/continuationItems")
        .or_else(|| raw.pointer("/continuationContents/playlistVideoListContinuation/contents"))
        .and_then(|c| c.as_array());

    if let Some(item_list) = items {
        for item in item_list {
            if let Some(video) = parse_playlist_item_value(item) {
                result.videos.push(video);
            } else if let Some(cir) = item.get("continuationItemRenderer").or_else(|| item.get("continuationItemViewModel")) {
                result.continuation_token = cir.pointer("/continuationEndpoint/continuationCommand/token")
                    .or_else(|| cir.pointer("/continuationEndpoint/command/token"))
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string());
            }
        }
    }

    Ok(result)
}

fn parse_playlist_item_value(item: &Value) -> Option<PlaylistVideoItem> {
    if let Some(pvr) = item.get("playlistVideoRenderer") {
        return parse_playlist_video_renderer(pvr);
    }

    if let Some(lvm) = item.get("lockupViewModel") {
        let id = lvm.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId")
            .and_then(|v| v.as_str())?
            .to_string();

        let title = lvm.pointer("/metadata/lockupMetadataViewModel/title/content")
            .and_then(|t| t.as_str())
            .unwrap_or("Untitled")
            .to_string();

        let author = lvm.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text/content")
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let author_id = lvm.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text/commandRuns/0/onTap/innertubeCommand/browseEndpoint/browseId")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        let duration = lvm.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/overlays/0/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text")
            .or_else(|| lvm.pointer("/contentImage/thumbnailViewModel/overlays/0/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text"))
            .and_then(|d| d.as_str())
            .map(|s| s.to_string());

        let thumbnail = lvm.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/image/sources/0/url")
            .or_else(|| lvm.pointer("/contentImage/thumbnailViewModel/image/sources/0/url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let duration_ms = duration.as_deref().and_then(|d| {
            let parts: Vec<&str> = d.split(':').collect();
            if parts.len() == 2 {
                let m: u64 = parts[0].parse().ok()?;
                let s: u64 = parts[1].parse().ok()?;
                Some((m * 60 + s) * 1000)
            } else if parts.len() == 3 {
                let h: u64 = parts[0].parse().ok()?;
                let m: u64 = parts[1].parse().ok()?;
                let s: u64 = parts[2].parse().ok()?;
                Some((h * 3600 + m * 60 + s) * 1000)
            } else {
                None
            }
        });

        return Some(PlaylistVideoItem {
            id,
            title,
            author,
            author_id,
            duration,
            duration_ms,
            thumbnail,
            index: None,
            is_playable: true,
        });
    }

    None
}

fn parse_playlist_video_renderer(pvr: &Value) -> Option<PlaylistVideoItem> {
    let id = pvr.get("videoId").and_then(|v| v.as_str())?.to_string();

    let title = pvr.pointer("/title/runs/0/text")
        .or_else(|| pvr.pointer("/title/simpleText"))
        .and_then(|t| t.as_str())
        .unwrap_or("Untitled")
        .to_string();

    let author = pvr.pointer("/shortBylineText/runs/0/text")
        .and_then(|t| t.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let author_id = pvr.pointer("/shortBylineText/runs/0/navigationEndpoint/browseEndpoint/browseId")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let duration = pvr.pointer("/lengthText/simpleText")
        .or_else(|| pvr.pointer("/lengthText/runs/0/text"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let duration_ms = pvr.get("lengthSeconds")
        .and_then(|s| s.as_str())
        .and_then(|s| s.parse::<u64>().ok())
        .map(|sec| sec * 1000);

    let thumbnail = pvr.pointer("/thumbnail/thumbnails/0/url")
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    let index = pvr.pointer("/index/simpleText")
        .or_else(|| pvr.pointer("/index/runs/0/text"))
        .and_then(|i| i.as_str())
        .and_then(|i| i.parse::<u32>().ok());

    let is_playable = pvr.get("isPlayable").and_then(|p| p.as_bool()).unwrap_or(true);

    Some(PlaylistVideoItem {
        id,
        title,
        author,
        author_id,
        duration,
        duration_ms,
        thumbnail,
        index,
        is_playable,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_playlist_fixture() {
        let fixture = json!({
            "header": {
                "playlistHeaderRenderer": {
                    "title": { "runs": [{ "text": "My Top Playlist" }] },
                    "ownerText": { "runs": [{ "text": "Caya Dev", "navigationEndpoint": { "browseEndpoint": { "browseId": "UC12345" } } }] },
                    "numVideosText": { "runs": [{ "text": "25 videos" }] }
                }
            },
            "contents": {
                "twoColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "itemSectionRenderer": {
                                            "contents": [{
                                                "playlistVideoListRenderer": {
                                                    "contents": [
                                                        {
                                                            "playlistVideoRenderer": {
                                                                "videoId": "abc12345678",
                                                                "title": { "runs": [{ "text": "Sample Song" }] },
                                                                "shortBylineText": { "runs": [{ "text": "Sample Artist" }] },
                                                                "lengthText": { "simpleText": "3:45" },
                                                                "lengthSeconds": "225",
                                                                "isPlayable": true
                                                            }
                                                        },
                                                        {
                                                            "continuationItemRenderer": {
                                                                "continuationEndpoint": {
                                                                    "continuationCommand": {
                                                                        "token": "TOKEN_PAGE_2"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    ]
                                                }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            }
        });

        let playlist = parse_playlist_browse_response("PL1234", &fixture).unwrap();
        assert_eq!(playlist.title, "My Top Playlist");
        assert_eq!(playlist.author.as_deref(), Some("Caya Dev"));
        assert_eq!(playlist.author_id.as_deref(), Some("UC12345"));
        assert_eq!(playlist.video_count, Some(25));
        assert_eq!(playlist.videos.len(), 1);
        assert_eq!(playlist.videos[0].id, "abc12345678");
        assert_eq!(playlist.videos[0].title, "Sample Song");
        assert_eq!(playlist.videos[0].duration.as_deref(), Some("3:45"));
        assert_eq!(playlist.videos[0].duration_ms, Some(225000));
        assert_eq!(playlist.continuation_token.as_deref(), Some("TOKEN_PAGE_2"));
    }
}
