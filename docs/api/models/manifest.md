# Model: Manifest (`src/models/manifest.rs`)

```rust
use innertube_rs::models::manifest::{ManifestStream, ParsedManifest};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestStream {
    pub itag: Option<u32>,
    pub mime_type: String,
    pub codecs: Option<String>,
    pub bandwidth: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<f32>,
    pub audio_channels: Option<u8>,
    pub sample_rate: Option<u32>,
    pub url: String,
    pub is_live: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedManifest {
    pub hls_streams: Vec<ManifestStream>,
    pub dash_streams: Vec<ManifestStream>,
    pub is_live: bool,
}
```
