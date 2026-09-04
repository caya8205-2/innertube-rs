use serde::{Deserialize, Serialize};

/// Caption track metadata extracted from player response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptTrack {
    pub language_code: String,
    pub name: String,
    pub kind: Option<String>,
    pub base_url: String,
    pub is_translatable: bool,
    /// Legacy `vssId` (used for DASH text set uids).
    pub vss_id: Option<String>,
}

/// A timed segment of a transcript.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    pub start_ms: u64,
    pub duration_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

/// Full timed transcript for a video.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transcript {
    pub video_id: String,
    pub language: String,
    pub segments: Vec<TranscriptSegment>,
}

impl Transcript {
    /// Format milliseconds as SRT timestamp (HH:MM:SS,mmm).
    fn format_srt_time(ms: u64) -> String {
        let total_seconds = ms / 1000;
        let millis = ms % 1000;
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = total_seconds / 3600;
        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, millis)
    }

    /// Format milliseconds as WebVTT timestamp (HH:MM:SS.mmm).
    fn format_vtt_time(ms: u64) -> String {
        let total_seconds = ms / 1000;
        let millis = ms % 1000;
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = total_seconds / 3600;
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    }

    /// Export the transcript to standard SubRip Subtitle format (.srt).
    pub fn to_srt(&self) -> String {
        let mut out = String::new();
        for (i, seg) in self.segments.iter().enumerate() {
            out.push_str(&format!("{}\n", i + 1));
            out.push_str(&format!(
                "{} --> {}\n",
                Self::format_srt_time(seg.start_ms),
                Self::format_srt_time(seg.end_ms)
            ));
            out.push_str(&format!("{}\n\n", seg.text.trim()));
        }
        out
    }

    /// Export the transcript to WebVTT format (.vtt).
    pub fn to_vtt(&self) -> String {
        let mut out = String::from("WEBVTT\n\n");
        for (i, seg) in self.segments.iter().enumerate() {
            out.push_str(&format!("{}\n", i + 1));
            out.push_str(&format!(
                "{} --> {}\n",
                Self::format_vtt_time(seg.start_ms),
                Self::format_vtt_time(seg.end_ms)
            ));
            out.push_str(&format!("{}\n\n", seg.text.trim()));
        }
        out
    }

    /// Export the transcript to plain text without timestamps.
    pub fn to_plain_text(&self) -> String {
        self.segments
            .iter()
            .map(|s| s.text.trim())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
