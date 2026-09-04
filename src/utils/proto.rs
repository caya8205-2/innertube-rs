use base64::engine::general_purpose::{STANDARD, URL_SAFE};
use base64::Engine;
use prost::Message;
use rand::Rng;

use crate::error::{InnertubeError, Result};
use crate::models::comments::PostCommentSort;
use crate::models::search::{
    DurationFilter, FeatureFilter, SearchFilters, SearchPrioritize, SearchTypeFilter,
    UploadDateFilter,
};
use crate::proto::misc::{
    community_post_comments_param, community_post_comments_param_container, community_post_params,
    create_comment_params, get_comments_section_params, hashtag, notification_preferences,
    peform_comment_action_params, reel_sequence, search_filter, CommunityPostCommentsParam,
    CommunityPostCommentsParamContainer, CommunityPostParams, CreateCommentParams,
    GetCommentsSectionParams, Hashtag, NextParams, NotificationPreferences,
    PeformCommentActionParams, ReelSequence, SearchFilter, VisitorData,
};

/// Encode random visitor ID & timestamp into VisitorData protobuf,
/// then convert to URL-safe Base64 with percent-encoded padding (`%3D`).
pub fn encode_visitor_data(id: &str, timestamp: i32) -> String {
    let visitor_data = VisitorData {
        id: id.to_string(),
        timestamp,
    };

    let mut buf = Vec::with_capacity(visitor_data.encoded_len());
    visitor_data
        .encode(&mut buf)
        .expect("VisitorData encoding should not fail");

    let base64_str = URL_SAFE.encode(&buf);
    base64_str.replace('=', "%3D")
}

/// Decode URL-safe Base64 visitor data back to `VisitorData`.
pub fn decode_visitor_data(visitor_data: &str) -> Result<VisitorData> {
    let unescaped = visitor_data.replace("%3D", "=").replace("%3d", "=");

    let bytes = URL_SAFE
        .decode(unescaped.as_bytes())
        .map_err(|e| InnertubeError::Other(format!("Failed to decode base64 visitor data: {e}")))?;

    let decoded = VisitorData::decode(&bytes[..])?;
    Ok(decoded)
}

/// Encode the opaque `createCommentParams` payload required by `/comment/create_comment`.
///
/// This mirrors YouTube.js' `CreateCommentParams` protobuf followed by Base64 and
/// URI-component encoding. Passing a raw video ID is rejected by InnerTube.
pub fn encode_create_comment_params(video_id: &str) -> Result<String> {
    let params = CreateCommentParams {
        video_id: video_id.to_string(),
        params: Some(create_comment_params::Params { index: 0 }),
        number: 7,
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode create-comment params: {err}"))
    })?;

    let base64 = STANDARD.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode the browse parameters for a Community Post detail page.
pub fn encode_community_post_params(post_id: &str, channel_id: &str) -> Result<String> {
    let params = CommunityPostParams {
        f1: Some(community_post_params::Field1 {
            ucid1: channel_id.to_string(),
            post_id: post_id.to_string(),
            ucid2: channel_id.to_string(),
        }),
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode community-post params: {err}"))
    })?;

    let base64 = STANDARD.encode(buf).replace('+', "-").replace('/', "_");
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode the continuation token required by the Community Post comments endpoint.
pub fn encode_community_post_comments_continuation(
    post_id: &str,
    channel_id: &str,
    sort: PostCommentSort,
) -> Result<String> {
    let comment_data = community_post_comments_param::comment_data_container::CommentData {
        sort_by: sort.proto_value(),
        f0: 2,
        f1: 0,
        post_id: post_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let details = CommunityPostCommentsParam {
        title: "posts".to_string(),
        comment_data_container: Some(community_post_comments_param::CommentDataContainer {
            comment_data: Some(comment_data),
            f0: 0,
            title: "comments-section".to_string(),
        }),
    };

    let mut details_buf = Vec::with_capacity(details.encoded_len());
    details.encode(&mut details_buf).map_err(|err| {
        InnertubeError::Other(format!(
            "Failed to encode community-post comment details: {err}"
        ))
    })?;
    let details_base64 = STANDARD
        .encode(details_buf)
        .replace('+', "-")
        .replace('/', "_");
    let proto_data: String =
        url::form_urlencoded::byte_serialize(details_base64.as_bytes()).collect();

    let container = CommunityPostCommentsParamContainer {
        f0: Some(community_post_comments_param_container::Container {
            location: "FEcomment_post_detail_page_web_top_level".to_string(),
            proto_data,
        }),
    };
    let mut container_buf = Vec::with_capacity(container.encoded_len());
    container.encode(&mut container_buf).map_err(|err| {
        InnertubeError::Other(format!(
            "Failed to encode community-post comment continuation: {err}"
        ))
    })?;

    let continuation = STANDARD.encode(container_buf);
    Ok(url::form_urlencoded::byte_serialize(continuation.as_bytes()).collect())
}

/// Encode `SearchFilters` into URL-encoded base64 `SearchFilter` protobuf params.
pub fn encode_search_filters(filters: &SearchFilters) -> Result<String> {
    let mut proto_filter = SearchFilter::default();

    if let Some(p) = filters.prioritize {
        proto_filter.prioritize = Some(match p {
            SearchPrioritize::Relevance => search_filter::Prioritize::Relevance as i32,
            SearchPrioritize::Popularity => search_filter::Prioritize::Popularity as i32,
        });
    }

    let mut proto_filters = search_filter::Filters::default();
    let mut has_sub_filters = false;

    if let Some(upload_date) = filters.upload_date {
        proto_filters.upload_date = Some(match upload_date {
            UploadDateFilter::All => search_filter::filters::UploadDate::AnyDate as i32,
            UploadDateFilter::Today => search_filter::filters::UploadDate::Today as i32,
            UploadDateFilter::Week => search_filter::filters::UploadDate::Week as i32,
            UploadDateFilter::Month => search_filter::filters::UploadDate::Month as i32,
            UploadDateFilter::Year => search_filter::filters::UploadDate::Year as i32,
        });
        has_sub_filters = true;
    }

    if let Some(search_type) = filters.search_type {
        proto_filters.r#type = Some(match search_type {
            SearchTypeFilter::All => search_filter::filters::SearchType::AnyType as i32,
            SearchTypeFilter::Video => search_filter::filters::SearchType::Video as i32,
            SearchTypeFilter::Shorts => search_filter::filters::SearchType::Shorts as i32,
            SearchTypeFilter::Channel => search_filter::filters::SearchType::Channel as i32,
            SearchTypeFilter::Playlist => search_filter::filters::SearchType::Playlist as i32,
            SearchTypeFilter::Movie => search_filter::filters::SearchType::Movie as i32,
        });
        has_sub_filters = true;
    }

    if let Some(duration) = filters.duration {
        proto_filters.duration = Some(match duration {
            DurationFilter::All => search_filter::filters::Duration::AnyDuration as i32,
            DurationFilter::OverTwentyMins => {
                search_filter::filters::Duration::OverTwentyMins as i32
            }
            DurationFilter::UnderThreeMins => {
                search_filter::filters::Duration::UnderThreeMins as i32
            }
            DurationFilter::ThreeToTwentyMins => {
                search_filter::filters::Duration::ThreeToTwentyMins as i32
            }
        });
        has_sub_filters = true;
    }

    for feature in &filters.features {
        has_sub_filters = true;
        match feature {
            FeatureFilter::Hd => proto_filters.features_hd = Some(true),
            FeatureFilter::Subtitles => proto_filters.features_subtitles = Some(true),
            FeatureFilter::CreativeCommons => proto_filters.features_creative_commons = Some(true),
            FeatureFilter::Feature3d => proto_filters.features_3d = Some(true),
            FeatureFilter::Live => proto_filters.features_live = Some(true),
            FeatureFilter::Purchased => proto_filters.features_purchased = Some(true),
            FeatureFilter::Feature4k => proto_filters.features_4k = Some(true),
            FeatureFilter::Feature360 => proto_filters.features_360 = Some(true),
            FeatureFilter::Location => proto_filters.features_location = Some(true),
            FeatureFilter::Hdr => proto_filters.features_hdr = Some(true),
            FeatureFilter::Vr180 => proto_filters.features_vr180 = Some(true),
        }
    }

    if has_sub_filters {
        proto_filter.filters = Some(proto_filters);
    }

    let mut buf = Vec::with_capacity(proto_filter.encoded_len());
    proto_filter.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode search filter: {err}"))
    })?;

    let base64 = STANDARD.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode `GetCommentsSectionParams` into URL-encoded base64 continuation token.
pub fn encode_comments_section_params(
    video_id: &str,
    sort: PostCommentSort,
    comment_id: Option<&str>,
) -> Result<String> {
    let params = GetCommentsSectionParams {
        ctx: Some(get_comments_section_params::Context {
            video_id: video_id.to_string(),
        }),
        unk_param: 6,
        params: Some(get_comments_section_params::Params {
            unk_token: None,
            opts: Some(get_comments_section_params::params::Options {
                video_id: video_id.to_string(),
                sort_by: sort.proto_value(),
                r#type: 2,
                comment_id: comment_id.map(|s| s.to_string()),
            }),
            replies_opts: None,
            page: None,
            target: "comments-section".to_string(),
        }),
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode comments section params: {err}"))
    })?;

    let base64 = STANDARD.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode `ReelSequence` into URL-encoded base64 sequenceParams.
pub fn encode_reel_sequence_params(short_id: &str) -> Result<String> {
    let params = ReelSequence {
        short_id: short_id.to_string(),
        params: Some(reel_sequence::Params { number: 5 }),
        feature_2: 25,
        feature_3: 0,
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode reel sequence params: {err}"))
    })?;

    let base64 = STANDARD.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode `NotificationPreferences` into URL-encoded base64 params.
pub fn encode_notification_preferences(channel_id: &str, pref_index: i32) -> Result<String> {
    let params = NotificationPreferences {
        channel_id: channel_id.to_string(),
        pref_id: Some(notification_preferences::Preference {
            index: pref_index,
        }),
        number_0: Some(0),
        number_1: Some(4),
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode notification preferences: {err}"))
    })?;

    let base64 = STANDARD.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode `Hashtag` into URL-safe Base64 and URL-encoded params.
pub fn encode_hashtag_params(hashtag: &str) -> Result<String> {
    let clean_hashtag = hashtag.trim_start_matches('#');
    let params = Hashtag {
        params: Some(hashtag::Params {
            hashtag: clean_hashtag.to_string(),
            r#type: 1,
        }),
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode hashtag params: {err}"))
    })?;

    let base64 = URL_SAFE.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Generate random alphanumeric string of given length.
/// Arguments for [`encode_comment_action_params`] (legacy
/// `CommentActionParamsArgs`).
#[derive(Debug, Clone, Default)]
pub struct CommentActionParamsArgs {
    pub comment_id: Option<String>,
    pub video_id: Option<String>,
    pub text: Option<String>,
    pub target_language: Option<String>,
}

/// Encode `PeformCommentActionParams` for `/comment/perform_comment_action`,
/// mirroring legacy `ProtoUtils.encodeCommentActionParams`: STANDARD base64,
/// URI-encoded. When `text` is present, `target_language` is required and the
/// translate params are attached; `unk_num` is dropped when a `comment_id` is
/// given.
pub fn encode_comment_action_params(
    action_type: i32,
    args: &CommentActionParamsArgs,
) -> Result<String> {
    let translate_comment_params = if let Some(ref text) = args.text {
        let target_language = args.target_language.clone().ok_or_else(|| {
            InnertubeError::Other("target_language must be a string".to_string())
        })?;
        Some(peform_comment_action_params::TranslateCommentParams {
            params: Some(peform_comment_action_params::translate_comment_params::Params {
                comment: Some(
                    peform_comment_action_params::translate_comment_params::params::Comment {
                        text: text.clone(),
                    },
                ),
            }),
            comment_id: args.comment_id.clone().unwrap_or_else(|| " ".to_string()),
            target_language,
        })
    } else {
        None
    };

    let data = PeformCommentActionParams {
        r#type: action_type,
        comment_id: args.comment_id.clone().unwrap_or_else(|| " ".to_string()),
        video_id: args.video_id.clone().unwrap_or_else(|| " ".to_string()),
        unk_num: if args.comment_id.is_some() && args.text.is_some() {
            None
        } else {
            Some(2)
        },
        channel_id: Some(" ".to_string()),
        translate_comment_params,
    };

    let mut buf = Vec::with_capacity(data.encoded_len());
    data.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode comment-action params: {err}"))
    })?;

    let base64 = STANDARD.encode(buf);
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

/// Encode `NextParams` for `/next` playlist navigation, mirroring legacy
/// `ProtoUtils.encodeNextParams`: URL-safe base64, URI-encoded.
pub fn encode_next_params(video_ids: &[&str], playlist_title: Option<&str>) -> Result<String> {
    let params = NextParams {
        video_id: video_ids.iter().map(ToString::to_string).collect(),
        playlist_title: playlist_title.map(ToString::to_string),
    };

    let mut buf = Vec::with_capacity(params.encoded_len());
    params.encode(&mut buf).map_err(|err| {
        InnertubeError::Other(format!("Failed to encode next params: {err}"))
    })?;

    let base64 = STANDARD.encode(buf).replace('+', "-").replace('/', "_");
    Ok(url::form_urlencoded::byte_serialize(base64.as_bytes()).collect())
}

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uri_unescape(encoded: &str) -> Vec<u8> {
        // Decode %XX sequences (form_urlencoded byte_serialize output).
        let bytes = encoded.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 3 <= bytes.len() {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap();
                out.push(u8::from_str_radix(hex, 16).unwrap());
                i += 3;
            } else {
                out.push(bytes[i]);
                i += 1;
            }
        }
        out
    }

    #[test]
    fn comment_action_params_translate_matches_golden_wire_bytes() {
        let args = CommentActionParamsArgs {
            comment_id: Some("cid".to_string()),
            video_id: Some("vid".to_string()),
            text: Some("hello".to_string()),
            target_language: Some("id".to_string()),
        };
        let encoded = encode_comment_action_params(22, &args).unwrap();

        let bytes = STANDARD.decode(uri_unescape(&encoded)).unwrap();

        // Hand-assembled golden vector (field order is deterministic):
        //   type=22 (f1), comment_id="cid" (f3), video_id="vid" (f5),
        //   channel_id=" " (f23), unk_num dropped (comment_id+text present),
        //   translate_comment_params (f31) { comment_id=f2, params=f3
        //     { comment=f1 { text=f1 "hello" } }, target_language=f4 "id" }
        let expected: &[u8] = &[
            0x08, 0x16, // f1 type = 22
            0x1A, 0x03, b'c', b'i', b'd', // f3 comment_id
            0x2A, 0x03, b'v', b'i', b'd', // f5 video_id
            0xBA, 0x01, 0x01, b' ', // f23 channel_id
            0xFA, 0x01, 0x14, // f31 translate_comment_params (len 20)
            0x12, 0x03, b'c', b'i', b'd', //   f2 comment_id
            0x1A, 0x09, //   f3 params (len 9)
            0x0A, 0x07, //     f1 comment (len 7)
            0x0A, 0x05, b'h', b'e', b'l', b'l', b'o', //       f1 text
            0x22, 0x02, b'i', b'd', //   f4 target_language
        ];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn comment_action_params_without_text_keeps_unk_num() {
        let args = CommentActionParamsArgs {
            comment_id: Some("cid".to_string()),
            video_id: Some("vid".to_string()),
            text: None,
            target_language: None,
        };
        let encoded = encode_comment_action_params(4, &args).unwrap();
        let bytes = STANDARD.decode(uri_unescape(&encoded)).unwrap();
        let decoded = PeformCommentActionParams::decode(&bytes[..]).unwrap();

        assert_eq!(decoded.r#type, 4);
        assert_eq!(decoded.unk_num, Some(2));
        assert!(decoded.translate_comment_params.is_none());
        assert_eq!(decoded.channel_id.as_deref(), Some(" "));
    }

    #[test]
    fn comment_action_params_translate_requires_target_language() {
        let args = CommentActionParamsArgs {
            text: Some("hello".to_string()),
            ..Default::default()
        };
        assert!(encode_comment_action_params(22, &args).is_err());
    }

    #[test]
    fn next_params_url_safe_chain_and_roundtrip() {
        let encoded = encode_next_params(&["aa", "bb"], Some("Mix")).unwrap();
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));

        let unescaped = String::from_utf8(uri_unescape(&encoded)).unwrap();
        let bytes = URL_SAFE.decode(unescaped.as_bytes()).unwrap();
        let decoded = NextParams::decode(&bytes[..]).unwrap();

        assert_eq!(decoded.video_id, vec!["aa".to_string(), "bb".to_string()]);
        assert_eq!(decoded.playlist_title.as_deref(), Some("Mix"));
    }

    #[test]
    fn test_visitor_data_roundtrip() {
        let id = "test_visitor_id";
        let timestamp = 1700000000;

        let encoded = encode_visitor_data(id, timestamp);
        assert!(!encoded.contains('='));

        let decoded = decode_visitor_data(&encoded).expect("Should decode successfully");
        assert_eq!(decoded.id, id);
        assert_eq!(decoded.timestamp, timestamp);
    }

    #[test]
    fn test_create_comment_params_match_legacy_contract() {
        let encoded = encode_create_comment_params("dQw4w9WgXcQ").expect("params should encode");
        assert!(encoded.contains("%3D"));

        let query = format!("value={encoded}");
        let base64: String = url::form_urlencoded::parse(query.as_bytes())
            .next()
            .map(|(_, value)| value.into_owned())
            .expect("URI component should decode");
        let bytes = STANDARD.decode(base64).expect("base64 should decode");
        let decoded =
            CreateCommentParams::decode(bytes.as_slice()).expect("protobuf should decode");

        assert_eq!(decoded.video_id, "dQw4w9WgXcQ");
        assert_eq!(decoded.params.map(|params| params.index), Some(0));
        assert_eq!(decoded.number, 7);
    }

    #[test]
    fn test_community_post_params_match_legacy_contract() {
        let encoded = encode_community_post_params("UgkxMjM0NTY3ODkw", "UC_test")
            .expect("params should encode");
        let query = format!("value={encoded}");
        let base64 = url::form_urlencoded::parse(query.as_bytes())
            .next()
            .map(|(_, value)| value.into_owned())
            .expect("URI component should decode")
            .replace('-', "+")
            .replace('_', "/");
        let bytes = STANDARD.decode(base64).expect("base64 should decode");
        let decoded =
            CommunityPostParams::decode(bytes.as_slice()).expect("protobuf should decode");
        let field = decoded.f1.expect("field should be present");

        assert_eq!(field.post_id, "UgkxMjM0NTY3ODkw");
        assert_eq!(field.ucid1, "UC_test");
        assert_eq!(field.ucid2, "UC_test");
    }

    #[test]
    fn test_search_filters_encode_match_legacy_contract() {
        let filters = SearchFilters {
            prioritize: Some(SearchPrioritize::Popularity),
            upload_date: Some(UploadDateFilter::Today),
            search_type: Some(SearchTypeFilter::Video),
            duration: Some(DurationFilter::UnderThreeMins),
            features: vec![FeatureFilter::Hd, FeatureFilter::Subtitles],
        };
        let encoded = encode_search_filters(&filters).expect("search filters should encode");
        let query = format!("value={encoded}");
        let base64: String = url::form_urlencoded::parse(query.as_bytes())
            .next()
            .map(|(_, value)| value.into_owned())
            .expect("URI component should decode");
        let bytes = STANDARD.decode(base64).expect("base64 should decode");
        let decoded = SearchFilter::decode(bytes.as_slice()).expect("protobuf should decode");

        assert_eq!(decoded.prioritize, Some(3));
        let sub = decoded.filters.expect("sub-filters should be present");
        assert_eq!(sub.upload_date, Some(2));
        assert_eq!(sub.r#type, Some(1));
        assert_eq!(sub.duration, Some(4));
        assert_eq!(sub.features_hd, Some(true));
        assert_eq!(sub.features_subtitles, Some(true));
        assert_eq!(sub.features_4k, None);
    }

    #[test]
    fn test_comments_section_params_encode_match_legacy_contract() {
        let encoded = encode_comments_section_params(
            "dQw4w9WgXcQ",
            PostCommentSort::NewestFirst,
            Some("comment_123"),
        )
        .expect("comments section params should encode");
        let query = format!("value={encoded}");
        let base64: String = url::form_urlencoded::parse(query.as_bytes())
            .next()
            .map(|(_, value)| value.into_owned())
            .expect("URI component should decode");
        let bytes = STANDARD.decode(base64).expect("base64 should decode");
        let decoded =
            GetCommentsSectionParams::decode(bytes.as_slice()).expect("protobuf should decode");

        assert_eq!(
            decoded.ctx.map(|ctx| ctx.video_id),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(decoded.unk_param, 6);
        let params = decoded.params.expect("params should be present");
        assert_eq!(params.target, "comments-section");
        let opts = params.opts.expect("opts should be present");
        assert_eq!(opts.video_id, "dQw4w9WgXcQ");
        assert_eq!(opts.sort_by, 1);
        assert_eq!(opts.r#type, 2);
        assert_eq!(opts.comment_id, Some("comment_123".to_string()));
    }

    #[test]
    fn test_reel_sequence_encode_match_legacy_contract() {
        let encoded =
            encode_reel_sequence_params("short_video_id").expect("reel sequence should encode");
        let query = format!("value={encoded}");
        let base64: String = url::form_urlencoded::parse(query.as_bytes())
            .next()
            .map(|(_, value)| value.into_owned())
            .expect("URI component should decode");
        let bytes = STANDARD.decode(base64).expect("base64 should decode");
        let decoded = ReelSequence::decode(bytes.as_slice()).expect("protobuf should decode");

        assert_eq!(decoded.short_id, "short_video_id");
        assert_eq!(decoded.params.map(|params| params.number), Some(5));
        assert_eq!(decoded.feature_2, 25);
        assert_eq!(decoded.feature_3, 0);
    }

    #[test]
    fn test_notification_preferences_encode_match_legacy_contract() {
        let encoded = encode_notification_preferences("UC_channel_123", 2)
            .expect("notification preferences should encode");
        let query = format!("value={encoded}");
        let base64: String = url::form_urlencoded::parse(query.as_bytes())
            .next()
            .map(|(_, value)| value.into_owned())
            .expect("URI component should decode");
        let bytes = STANDARD.decode(base64).expect("base64 should decode");
        let decoded =
            NotificationPreferences::decode(bytes.as_slice()).expect("protobuf should decode");

        assert_eq!(decoded.channel_id, "UC_channel_123");
        assert_eq!(decoded.pref_id.map(|p| p.index), Some(2));
        assert_eq!(decoded.number_0, Some(0));
        assert_eq!(decoded.number_1, Some(4));
    }
}
