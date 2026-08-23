use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use prost::Message;
use rand::Rng;

use crate::error::{InnertubeError, Result};
use crate::proto::misc::VisitorData;

/// Encode random visitor ID & timestamp into VisitorData protobuf,
/// then convert to URL-safe Base64 with percent-encoded padding (`%3D`).
pub fn encode_visitor_data(id: &str, timestamp: i32) -> String {
    let visitor_data = VisitorData {
        id: id.to_string(),
        timestamp,
    };

    let mut buf = Vec::with_capacity(visitor_data.encoded_len());
    visitor_data.encode(&mut buf).expect("VisitorData encoding should not fail");

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
}
