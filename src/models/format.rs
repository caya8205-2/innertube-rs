use serde::{Deserialize, Serialize};

/// Percent-decode a string (legacy `decodeURIComponent`; base64 payloads are
/// ASCII so byte-to-char mapping is safe here).
fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 3 <= bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                    i += 3;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Audio track metadata (legacy `AudioTrack`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    pub audio_is_default: bool,
    pub display_name: String,
    pub id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingFormat {
    pub itag: u32,
    pub url: Option<String>,
    pub signature_cipher: Option<String>,
    pub cipher: Option<String>,
    pub mime_type: String,
    pub bitrate: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub quality_label: Option<String>,
    pub audio_quality: Option<String>,
    pub approx_duration_ms: Option<String>,
    pub audio_sample_rate: Option<String>,
    pub audio_channels: Option<u32>,
    pub content_length: Option<String>,
    pub average_bitrate: Option<u64>,
    pub audio_bitrate: Option<u32>,
    pub audio_track: Option<AudioTrack>,
    pub xtags: Option<String>,
    pub is_drc: Option<bool>,
    pub is_vb: Option<bool>,
    pub index_range: Option<FormatRange>,
    pub init_range: Option<FormatRange>,
    pub target_duration_sec: Option<f64>,
    pub is_type_otf: Option<bool>,
    pub color_info: Option<FormatColorInfo>,
    pub fps: Option<u32>,
    pub drm_families: Option<Vec<String>>,
    pub drm_track_type: Option<String>,
}

/// Byte range of a format (legacy `Range`; InnerTube sends string offsets).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatRange {
    pub start: String,
    pub end: String,
}

/// Raw color info from a format (pre-CICP mapping).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatColorInfo {
    pub primaries: Option<String>,
    pub transfer_characteristics: Option<String>,
    pub matrix_coefficients: Option<String>,
}

impl StreamingFormat {
    pub fn is_audio_only(&self) -> bool {
        self.mime_type.starts_with("audio/")
    }

    pub fn is_video_only(&self) -> bool {
        self.mime_type.starts_with("video/") && self.audio_quality.is_none()
    }

    pub fn is_audio_video(&self) -> bool {
        self.mime_type.starts_with("video/") && self.audio_quality.is_some()
    }

    /// Legacy `has_audio`: audioBitrate or audioQuality present.
    pub fn has_audio(&self) -> bool {
        self.audio_bitrate.is_some() || self.audio_quality.is_some()
    }

    /// Legacy `has_video`: qualityLabel present.
    pub fn has_video(&self) -> bool {
        self.quality_label.is_some()
    }

    /// Legacy `has_text`: captionTrack present (not modeled; always false).
    pub fn has_text(&self) -> bool {
        false
    }

    /// Parsed `xtags` key/value pairs. `xtags` is a base64url-encoded
    /// protobuf `FormatXTags` message (legacy `Format.ts`).
    pub fn xtag_pairs(&self) -> Vec<(String, String)> {
        use base64::Engine;
        use prost::Message;

        let Some(ref xtags) = self.xtags else {
            return Vec::new();
        };
        // Legacy: decodeURIComponent, then URL-safe alphabet to standard.
        let decoded = percent_decode(xtags).replace('-', "+").replace('_', "/");
        let Ok(bytes) = base64::engine::general_purpose::URL_SAFE.decode(decoded) else {
            return Vec::new();
        };
        let Ok(parsed) = crate::proto::misc::FormatXTags::decode(&bytes[..]) else {
            return Vec::new();
        };

        parsed
            .xtags
            .into_iter()
            .map(|kv| (kv.key.unwrap_or_default(), kv.value.unwrap_or_default()))
            .collect()
    }

    fn xtag(&self, key: &str) -> Option<String> {
        self.xtag_pairs()
            .into_iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Legacy `language` (from the `lang` xtag).
    pub fn language(&self) -> Option<String> {
        if self.has_audio() || self.has_text() {
            self.xtag("lang")
        } else {
            None
        }
    }

    fn audio_content(&self) -> Option<String> {
        self.xtag("acont")
    }

    /// Legacy `is_drc` (dynamic range compression audio).
    pub fn is_drc(&self) -> bool {
        self.is_drc.unwrap_or(false)
            || self.xtag("drc").as_deref() == Some("1")
    }

    /// Legacy `is_vb` (descriptive video).
    pub fn is_vb(&self) -> bool {
        self.is_vb.unwrap_or(false)
            || self.xtag("vb").as_deref() == Some("1")
    }

    pub fn is_dubbed(&self) -> bool {
        self.audio_content().as_deref() == Some("dubbed")
    }

    pub fn is_descriptive(&self) -> bool {
        self.audio_content().as_deref() == Some("descriptive")
    }

    pub fn is_secondary(&self) -> bool {
        self.audio_content().as_deref() == Some("secondary")
    }

    pub fn is_auto_dubbed(&self) -> bool {
        self.audio_content().as_deref() == Some("dubbed-auto")
    }

    /// Legacy `is_original`.
    pub fn is_original(&self) -> bool {
        self.audio_content().as_deref() == Some("original")
            || (!self.is_dubbed()
                && !self.is_descriptive()
                && !self.is_secondary()
                && !self.is_auto_dubbed()
                && !self.is_drc())
    }

    pub fn get_raw_cipher_url(&self) -> Option<(String, Option<String>, Option<String>)> {
        if let Some(ref cipher_str) = self.signature_cipher.as_ref().or(self.cipher.as_ref()) {
            let parsed = url::Url::parse(&format!("http://localhost/?{cipher_str}")).ok()?;
            let mut url = None;
            let mut s = None;
            let mut sp = None;

            for (k, v) in parsed.query_pairs() {
                match k.as_ref() {
                    "url" => url = Some(v.to_string()),
                    "s" => s = Some(v.to_string()),
                    "sp" => sp = Some(v.to_string()),
                    _ => {}
                }
            }

            if let Some(u) = url {
                return Some((u, sp, s));
            }
        }

        self.url.as_ref().map(|u| (u.clone(), None, None))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormatType {
    AudioOnly,
    VideoOnly,
    AudioVideo,
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityPreference {
    Highest,
    Lowest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormatFilter {
    pub format_type: FormatType,
    pub quality: QualityPreference,
    pub container: Option<String>, // e.g. "mp4", "webm"
}

impl Default for FormatFilter {
    fn default() -> Self {
        Self {
            format_type: FormatType::AudioOnly,
            quality: QualityPreference::Highest,
            container: None,
        }
    }
}

/// Byte range for downloading a specific segment of a media stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadRange {
    pub start: u64,
    pub end: u64,
}

/// Rich format options matching YouTube.js FormatOptions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatOptions {
    pub client: Option<String>,
    pub po_token: Option<String>,
    pub itag: Option<u32>,
    pub quality: Option<String>,
    pub format_type: Option<FormatType>,
    pub format: Option<String>,
    pub codec: Option<String>,
    pub language: Option<String>,
}

/// Download options matching YouTube.js DownloadOptions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    #[serde(flatten)]
    pub format_options: FormatOptions,
    pub range: Option<DownloadRange>,
}
