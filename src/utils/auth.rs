use sha1::{Digest, Sha1};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::YOUTUBE_BASE_URL;

/// Generate a `SAPISIDHASH` authorization header value, mirroring
/// `Utils.generateSidAuth` in YouTube.js:
/// `SAPISIDHASH {ts}_{sha1("{ts} {sapisid} {origin}")}`.
pub fn generate_sid_auth(sapisid: &str) -> String {
    generate_sid_auth_at(sapisid, now_unix_seconds())
}

/// Deterministic core of [`generate_sid_auth`] for testing.
pub fn generate_sid_auth_at(sapisid: &str, timestamp: u64) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("{timestamp} {sapisid} {YOUTUBE_BASE_URL}").as_bytes());
    let digest = hex_encode(&hasher.finalize());
    format!("SAPISIDHASH {timestamp}_{digest}")
}

/// Extract a cookie value by name from a `Cookie` header string, mirroring
/// legacy `getCookie` (whole-name match).
pub fn get_cookie<'a>(cookies: &'a str, name: &str) -> Option<&'a str> {
    cookies.split(';').map(str::trim).find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then_some(value)
    })
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sid_auth_matches_known_sha1_vector() {
        // SHA-1("1700000000 test-sapisid-value https://www.youtube.com")
        assert_eq!(
            generate_sid_auth_at("test-sapisid-value", 1_700_000_000),
            "SAPISIDHASH 1700000000_5820a5e69f4feb3f2d6e421470411e3ece1dad14"
        );
    }

    #[test]
    fn get_cookie_matches_whole_names() {
        let cookies = "SID=abc; SAPISID=xyz123; HSID=zzz";
        assert_eq!(get_cookie(cookies, "SAPISID"), Some("xyz123"));
        assert_eq!(get_cookie(cookies, "SID"), Some("abc"));
        assert_eq!(get_cookie(cookies, "APISID"), None);
        assert_eq!(get_cookie(cookies, "MISSING"), None);
    }

    #[test]
    fn get_cookie_handles_first_and_last_positions() {
        assert_eq!(get_cookie("SAPISID=first; OTHER=1", "SAPISID"), Some("first"));
        assert_eq!(get_cookie("OTHER=1; SAPISID=last", "SAPISID"), Some("last"));
        assert_eq!(get_cookie("SAPISID=only", "SAPISID"), Some("only"));
        assert_eq!(get_cookie("NOT_SAPISID=x", "SAPISID"), None);
    }
}
