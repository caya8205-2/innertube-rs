# Endpoint: Transcript (`src/endpoints/transcript.rs`)

Fetches timed video transcripts and multi-language subtitle tracks.

```rust
use innertube_rs::endpoints::transcript::{get_transcript_tracks, get_transcript};
```

---

## Functions

### `get_transcript_tracks(session: &Session, video_id: &str) -> Result<Vec<TranscriptTrack>>`
Extracts all available caption tracks (ASR, translated, manual) from `/player`.

### `get_transcript(session: &Session, video_id: &str, lang: Option<&str>) -> Result<Transcript>`
Downloads and parses timed transcript segments into a `Transcript` struct with built-in export helpers:
* `transcript.to_srt()` $\rightarrow$ Generates standard SubRip `.srt` format.
* `transcript.to_vtt()` $\rightarrow$ Generates WebVTT `.vtt` format.
