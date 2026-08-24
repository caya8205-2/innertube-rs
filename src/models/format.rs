use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

/// Download options matching YouTube.js DownloadOptions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    #[serde(flatten)]
    pub format_options: FormatOptions,
    pub range: Option<DownloadRange>,
}
