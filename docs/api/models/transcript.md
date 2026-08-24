# Model: Transcript (`src/models/transcript.rs`)

```rust
use innertube_rs::models::transcript::{Transcript, TranscriptTrack, TranscriptSegment};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptTrack {
    pub language_code: String,
    pub language_name: String,
    pub kind: Option<String>,
    pub base_url: String,
    pub is_translatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub start_ms: u64,
    pub duration_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    pub video_id: String,
    pub language: String,
    pub segments: Vec<TranscriptSegment>,
}

impl Transcript {
    /// Formats segments into standard SubRip (.srt) subtitle string
    pub fn to_srt(&self) -> String;

    /// Formats segments into WebVTT (.vtt) subtitle string
    pub fn to_vtt(&self) -> String;
}
```
