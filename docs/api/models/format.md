# Model: Format (`src/models/format.rs`)

```rust
use innertube_rs::models::format::{Format, FormatFilter, FormatType, QualityPreference};
```

---

## Enums & Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub itag: u32,
    pub url: Option<String>,
    pub signature_cipher: Option<String>,
    pub cipher: Option<String>,
    pub mime_type: String,
    pub bitrate: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub quality_label: Option<String>,
    pub audio_quality: Option<String>,
    pub approx_duration_ms: Option<String>,
    pub audio_sample_rate: Option<String>,
    pub audio_channels: Option<u8>,
    pub content_length: Option<String>,
    pub fps: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatType {
    AudioOnly,
    VideoOnly,
    Both,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualityPreference {
    Highest,
    Lowest,
    Exact(u32),
}

#[derive(Debug, Clone)]
pub struct FormatFilter {
    pub format_type: FormatType,
    pub quality: QualityPreference,
    pub container: Option<String>,
}
```
