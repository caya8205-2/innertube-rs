use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::Result;
use crate::models::channel::{
    ChannelAbout, ChannelShortItem, ChannelShortsResponse, ChannelVideoItem, ChannelVideosResponse,
};

/// Fetch channel profile and about details.
pub async fn get_channel_about(session: &Session, channel_id: &str) -> Result<ChannelAbout> {
    let payload = json!({
        "browseId": channel_id,
    });

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_channel_about_response(channel_id, &raw)
}

/// Fetch channel videos (Videos tab) with pagination support.
pub async fn get_channel_videos(
    session: &Session,
    channel_id: &str,
    continuation_token: Option<&str>,
) -> Result<ChannelVideosResponse> {
    let payload = if let Some(token) = continuation_token {
        json!({
            "continuation": token,
        })
    } else {
        json!({
            "browseId": channel_id,
            "params": "EgZ2aWRlb3PyBgQKAjoA", // Videos tab
        })
    };

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_channel_videos_response(channel_id, &raw)
}

/// Fetch channel shorts (Shorts tab) with pagination support.
pub async fn get_channel_shorts(
    session: &Session,
    channel_id: &str,
    continuation_token: Option<&str>,
) -> Result<ChannelShortsResponse> {
    let payload = if let Some(token) = continuation_token {
        json!({
            "continuation": token,
        })
    } else {
        json!({
            "browseId": channel_id,
            "params": "EgZzaG9ydHPyBgUKA5oBAA%3D%3D", // Shorts tab
        })
    };

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_channel_shorts_response(channel_id, &raw)
}

/// Parse channel about / header response.
pub fn parse_channel_about_response(channel_id: &str, raw: &Value) -> Result<ChannelAbout> {
    let mut about = ChannelAbout {
        channel_id: channel_id.to_string(),
        ..Default::default()
    };

    // Header extraction
    let header = raw.pointer("/header/c4TabbedHeaderRenderer")
        .or_else(|| raw.pointer("/header/pageHeaderRenderer"));

    if let Some(h) = header {
        about.title = h.pointer("/title").and_then(|t| t.as_str())
            .or_else(|| h.pointer("/pageTitle").and_then(|t| t.as_str()))
            .or_else(|| h.pointer("/content/pageHeaderViewModel/title/dynamicTextViewModel/text/content").and_then(|t| t.as_str()))
            .unwrap_or("Unknown Channel")
            .to_string();

        about.subscriber_count = h.pointer("/subscriberCountText/simpleText")
            .or_else(|| h.pointer("/subscriberCountText/runs/0/text"))
            .or_else(|| h.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/0/text/content"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        about.avatar = h.pointer("/avatar/thumbnails/0/url")
            .or_else(|| h.pointer("/content/pageHeaderViewModel/image/decoratedAvatarViewModel/avatar/avatarViewModel/image/sources/0/url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        about.banner = h.pointer("/banner/thumbnails/0/url")
            .or_else(|| h.pointer("/content/pageHeaderViewModel/banner/bannerViewModel/image/sources/0/url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        about.custom_url = h.pointer("/channelHandleText/runs/0/text")
            .or_else(|| h.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text/content"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());
    }

    // Microformat / metadata
    if let Some(micro) = raw.pointer("/microformat/microformatDataRenderer") {
        if about.title.is_empty() || about.title == "Unknown Channel" {
            if let Some(t) = micro.get("title").and_then(|t| t.as_str()) {
                about.title = t.to_string();
            }
        }
        if about.description.is_none() {
            about.description = micro.get("description").and_then(|d| d.as_str()).map(|s| s.to_string());
        }
    }

    Ok(about)
}

/// Parse channel videos response into `ChannelVideosResponse`.
pub fn parse_channel_videos_response(channel_id: &str, raw: &Value) -> Result<ChannelVideosResponse> {
    let mut resp = ChannelVideosResponse {
        channel_id: channel_id.to_string(),
        ..Default::default()
    };

    let mut items_to_parse = Vec::new();

    // Direct continuation items
    if let Some(cont_items) = raw.pointer("/onResponseReceivedActions/0/appendContinuationItemsAction/continuationItems").and_then(|c| c.as_array()) {
        items_to_parse.extend(cont_items.iter());
    }

    // Tab contents
    if let Some(tabs) = raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs").and_then(|t| t.as_array()) {
        for tab in tabs {
            if let Some(contents) = tab.pointer("/tabRenderer/content/richGridRenderer/contents").and_then(|c| c.as_array()) {
                items_to_parse.extend(contents.iter());
            } else if let Some(contents) = tab.pointer("/tabRenderer/content/sectionListRenderer/contents/0/itemSectionRenderer/contents/0/gridRenderer/items").and_then(|c| c.as_array()) {
                items_to_parse.extend(contents.iter());
            }
        }
    }

    for item in items_to_parse {
        if let Some(lvm) = item.pointer("/richItemRenderer/content/lockupViewModel").or_else(|| item.get("lockupViewModel")) {
            let vid = lvm.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId")
                .and_then(|v| v.as_str());

            if let Some(v_id) = vid {
                let title = lvm.pointer("/metadata/lockupMetadataViewModel/title/content")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Untitled")
                    .to_string();

                let views = lvm.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text/content")
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string());

                let published_time = lvm.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/1/text/content")
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

                resp.videos.push(ChannelVideoItem {
                    video_id: v_id.to_string(),
                    title,
                    published_time,
                    duration,
                    views,
                    thumbnail,
                });
            }
        } else if let Some(vr) = item.get("videoRenderer").or_else(|| item.pointer("/richItemRenderer/content/videoRenderer")) {
            if let Some(vid) = vr.get("videoId").and_then(|v| v.as_str()) {
                let title = vr.pointer("/title/runs/0/text")
                    .or_else(|| vr.pointer("/title/simpleText"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("Untitled")
                    .to_string();

                let published_time = vr.pointer("/publishedTimeText/simpleText")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());

                let duration = vr.pointer("/lengthText/simpleText")
                    .and_then(|l| l.as_str())
                    .map(|s| s.to_string());

                let views = vr.pointer("/viewCountText/simpleText")
                    .or_else(|| vr.pointer("/viewCountText/runs/0/text"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let thumbnail = vr.pointer("/thumbnail/thumbnails/0/url")
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string());

                resp.videos.push(ChannelVideoItem {
                    video_id: vid.to_string(),
                    title,
                    published_time,
                    duration,
                    views,
                    thumbnail,
                });
            }
        } else if let Some(cir) = item.get("continuationItemRenderer").or_else(|| item.get("continuationItemViewModel")) {
            resp.continuation_token = cir.pointer("/continuationEndpoint/continuationCommand/token")
                .or_else(|| cir.pointer("/continuationEndpoint/command/token"))
                .and_then(|t| t.as_str())
                .map(|s| s.to_string());
        }
    }

    Ok(resp)
}

/// Parse channel shorts response into `ChannelShortsResponse`.
pub fn parse_channel_shorts_response(channel_id: &str, raw: &Value) -> Result<ChannelShortsResponse> {
    let mut resp = ChannelShortsResponse {
        channel_id: channel_id.to_string(),
        ..Default::default()
    };

    let mut items_to_parse = Vec::new();

    // Direct continuation items
    if let Some(cont_items) = raw.pointer("/onResponseReceivedActions/0/appendContinuationItemsAction/continuationItems").and_then(|c| c.as_array()) {
        items_to_parse.extend(cont_items.iter());
    }

    // Tab contents
    if let Some(tabs) = raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs").and_then(|t| t.as_array()) {
        for tab in tabs {
            if let Some(contents) = tab.pointer("/tabRenderer/content/richGridRenderer/contents").and_then(|c| c.as_array()) {
                items_to_parse.extend(contents.iter());
            }
        }
    }

    for item in items_to_parse {
        if let Some(slvm) = item.pointer("/richItemRenderer/content/shortsLockupViewModel").or_else(|| item.get("shortsLockupViewModel")) {
            let vid = slvm.pointer("/onTap/innertubeCommand/reelWatchEndpoint/videoId")
                .or_else(|| slvm.pointer("/onTap/innertubeCommand/watchEndpoint/videoId"))
                .and_then(|v| v.as_str());

            if let Some(v_id) = vid {
                let title = slvm.pointer("/overlayMetadata/primaryText/content")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Untitled Short")
                    .to_string();

                let views = slvm.pointer("/overlayMetadata/secondaryText/content")
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string());

                let thumbnail = slvm.pointer("/thumbnailViewModel/thumbnailViewModel/image/sources/0/url")
                    .or_else(|| slvm.pointer("/thumbnailViewModel/image/sources/0/url"))
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string());

                resp.shorts.push(ChannelShortItem {
                    video_id: v_id.to_string(),
                    title,
                    views,
                    thumbnail,
                });
            }
        } else if let Some(lvm) = item.pointer("/richItemRenderer/content/lockupViewModel").or_else(|| item.get("lockupViewModel")) {
            let vid = lvm.pointer("/rendererContext/commandContext/onTap/innertubeCommand/reelWatchEndpoint/videoId")
                .or_else(|| lvm.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId"))
                .and_then(|v| v.as_str());

            if let Some(v_id) = vid {
                let title = lvm.pointer("/metadata/lockupMetadataViewModel/title/content")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Untitled Short")
                    .to_string();

                let views = lvm.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text/content")
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string());

                let thumbnail = lvm.pointer("/contentImage/thumbnailViewModel/image/sources/0/url")
                    .or_else(|| lvm.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/image/sources/0/url"))
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string());

                resp.shorts.push(ChannelShortItem {
                    video_id: v_id.to_string(),
                    title,
                    views,
                    thumbnail,
                });
            }
        } else if let Some(rir) = item.get("reelItemRenderer").or_else(|| item.pointer("/richItemRenderer/content/reelItemRenderer")) {
            if let Some(vid) = rir.get("videoId").and_then(|v| v.as_str()) {
                let title = rir.pointer("/headline/simpleText")
                    .or_else(|| rir.pointer("/headline/runs/0/text"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("Untitled Short")
                    .to_string();

                let views = rir.pointer("/viewCountText/simpleText")
                    .or_else(|| rir.pointer("/viewCountText/runs/0/text"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let thumbnail = rir.pointer("/thumbnail/thumbnails/0/url")
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string());

                resp.shorts.push(ChannelShortItem {
                    video_id: vid.to_string(),
                    title,
                    views,
                    thumbnail,
                });
            }
        } else if let Some(cir) = item.get("continuationItemRenderer").or_else(|| item.get("continuationItemViewModel")) {
            resp.continuation_token = cir.pointer("/continuationEndpoint/continuationCommand/token")
                .or_else(|| cir.pointer("/continuationEndpoint/command/token"))
                .and_then(|t| t.as_str())
                .map(|s| s.to_string());
        }
    }

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_channel_about_fixture() {
        let fixture = json!({
            "header": {
                "c4TabbedHeaderRenderer": {
                    "title": "YOASOBI",
                    "subscriberCountText": { "simpleText": "6.5M subscribers" },
                    "avatar": { "thumbnails": [{ "url": "https://avatar.jpg" }] },
                    "banner": { "thumbnails": [{ "url": "https://banner.jpg" }] }
                }
            },
            "microformat": {
                "microformatDataRenderer": {
                    "description": "Official YOASOBI Channel."
                }
            }
        });

        let about = parse_channel_about_response("UCbqY3RHKkPS8dJCrfAfSk6Q", &fixture).unwrap();
        assert_eq!(about.title, "YOASOBI");
        assert_eq!(about.subscriber_count.as_deref(), Some("6.5M subscribers"));
        assert_eq!(about.description.as_deref(), Some("Official YOASOBI Channel."));
        assert_eq!(about.avatar.as_deref(), Some("https://avatar.jpg"));
        assert_eq!(about.banner.as_deref(), Some("https://banner.jpg"));
    }
}
