use base64::engine::general_purpose::{STANDARD, URL_SAFE};
use base64::Engine;
use prost::Message;
use rand::Rng;

use crate::error::{InnertubeError, Result};
use crate::models::comments::PostCommentSort;
use crate::proto::misc::{
    community_post_comments_param, community_post_comments_param_container, community_post_params,
    create_comment_params, CommunityPostCommentsParam, CommunityPostCommentsParamContainer,
    CommunityPostParams, CreateCommentParams, VisitorData,
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

/// Generate random alphanumeric string of given length.
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
}
