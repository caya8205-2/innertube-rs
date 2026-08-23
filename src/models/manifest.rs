use serde::{Deserialize, Serialize};

/// Individual stream representation extracted from an HLS (.m3u8) or DASH (.mpd) manifest.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManifestStream {
    pub itag: Option<u32>,
    pub mime_type: String,
    pub codecs: Option<String>,
    pub bandwidth: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<f32>,
    pub audio_channels: Option<u8>,
    pub sample_rate: Option<u32>,
    pub url: String,
    pub is_live: bool,
}

/// Consolidated parsed representations from media manifests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParsedManifest {
    pub streams: Vec<ManifestStream>,
    pub is_live: bool,
}
