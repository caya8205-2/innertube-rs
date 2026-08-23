use serde_json::Value;
use crate::core::session::Session;
use crate::endpoints::player::fetch_player_response;
use crate::error::{InnertubeError, Result};
use crate::models::transcript::{Transcript, TranscriptSegment, TranscriptTrack};

/// Extract available caption tracks for a video from the InnerTube `/player` endpoint.
pub async fn get_transcript_tracks(session: &Session, video_id: &str) -> Result<Vec<TranscriptTrack>> {
    let player_response = fetch_player_response(session, video_id, None).await?;
    extract_caption_tracks_from_player(&serde_json::to_value(&player_response).map_err(InnertubeError::Json)?)
}

/// Extract caption tracks directly from a player JSON response.
pub fn extract_caption_tracks_from_player(player_json: &Value) -> Result<Vec<TranscriptTrack>> {
    let mut tracks = Vec::new();

    let caption_tracks = match player_json.pointer("/captions/playerCaptionsTracklistRenderer/captionTracks").and_then(|c| c.as_array()) {
        Some(arr) => arr,
        None => return Ok(tracks),
    };

    for item in caption_tracks {
        let base_url = match item.get("baseUrl").and_then(|u| u.as_str()) {
            Some(u) => u.to_string(),
            None => continue,
        };

        let language_code = item.get("languageCode")
            .and_then(|l| l.as_str())
            .unwrap_or("unknown")
            .to_string();

        let name = item.pointer("/name/simpleText")
            .or_else(|| item.pointer("/name/runs/0/text"))
            .and_then(|t| t.as_str())
            .unwrap_or(&language_code)
            .to_string();

        let kind = item.get("kind").and_then(|k| k.as_str()).map(|k| k.to_string());
        let is_translatable = item.get("isTranslatable").and_then(|t| t.as_bool()).unwrap_or(false);

        tracks.push(TranscriptTrack {
            language_code,
            name,
            kind,
            base_url,
            is_translatable,
        });
    }

    Ok(tracks)
}

/// Fetch timed transcript for a video ID in the requested language (or default first track).
pub async fn get_transcript(session: &Session, video_id: &str, lang: Option<&str>) -> Result<Transcript> {
    let tracks = get_transcript_tracks(session, video_id).await?;
    if tracks.is_empty() {
        return Err(InnertubeError::Other(format!(
            "No caption tracks found for video ID: {}",
            video_id
        )));
    }

    // Select matching language track, or first track
    let track = if let Some(target_lang) = lang {
        tracks
            .iter()
            .find(|t| t.language_code.eq_ignore_ascii_case(target_lang) || t.name.to_lowercase().contains(&target_lang.to_lowercase()))
            .or_else(|| tracks.first())
            .ok_or_else(|| InnertubeError::Other(format!("No matching caption track for language: {}", target_lang)))?
    } else {
        tracks.first().unwrap()
    };

    // Append JSON3 format parameter to get structured JSON instead of XML
    let timedtext_url = if track.base_url.contains("fmt=") {
        track.base_url.clone()
    } else {
        format!("{}&fmt=json3", track.base_url)
    };

    let resp = session.http_client.get(&timedtext_url).send().await.map_err(InnertubeError::Network)?;
    let text = resp.text().await.map_err(InnertubeError::Network)?;

    parse_transcript_response(video_id, &track.language_code, &text)
}

/// Parse YouTube timedtext response (supports both JSON3 and XML formats).
pub fn parse_transcript_response(video_id: &str, language: &str, body: &str) -> Result<Transcript> {
    let mut segments = Vec::new();

    // 1. Try parsing as JSON3
    if let Ok(json) = serde_json::from_str::<Value>(body) {
        if let Some(events) = json.get("events").and_then(|e| e.as_array()) {
            for ev in events {
                let start_ms = ev.get("tStartMs").and_then(|t| t.as_u64()).unwrap_or(0);
                let duration_ms = ev.get("dDurationMs").and_then(|d| d.as_u64()).unwrap_or(0);
                let end_ms = start_ms + duration_ms;

                let mut text = String::new();
                if let Some(segs) = ev.get("segs").and_then(|s| s.as_array()) {
                    for seg in segs {
                        if let Some(utf8) = seg.get("utf8").and_then(|u| u.as_str()) {
                            text.push_str(utf8);
                        }
                    }
                }

                let cleaned_text = text.trim().to_string();
                if !cleaned_text.is_empty() {
                    segments.push(TranscriptSegment {
                        start_ms,
                        duration_ms,
                        end_ms,
                        text: cleaned_text,
                    });
                }
            }

            return Ok(Transcript {
                video_id: video_id.to_string(),
                language: language.to_string(),
                segments,
            });
        }
    }

    // 2. Fallback XML parsing (<text start="1.23" dur="4.56">...</text>)
    for line in body.split("<text") {
        if let Some(end_tag) = line.find("</text>") {
            let chunk = &line[..end_tag];
            let start_sec = extract_xml_attr(chunk, "start").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let dur_sec = extract_xml_attr(chunk, "dur").and_then(|d| d.parse::<f64>().ok()).unwrap_or(0.0);

            let content_start = chunk.find('>').map(|i| i + 1).unwrap_or(0);
            let raw_text = &chunk[content_start..];
            let decoded_text = decode_xml_entities(raw_text).trim().to_string();

            if !decoded_text.is_empty() {
                let start_ms = (start_sec * 1000.0) as u64;
                let duration_ms = (dur_sec * 1000.0) as u64;
                segments.push(TranscriptSegment {
                    start_ms,
                    duration_ms,
                    end_ms: start_ms + duration_ms,
                    text: decoded_text,
                });
            }
        }
    }

    Ok(Transcript {
        video_id: video_id.to_string(),
        language: language.to_string(),
        segments,
    })
}

fn extract_xml_attr<'a>(chunk: &'a str, attr_name: &str) -> Option<&'a str> {
    let pattern = format!("{}=\"", attr_name);
    let start = chunk.find(&pattern)? + pattern.len();
    let end = chunk[start..].find('"')? + start;
    Some(&chunk[start..end])
}

fn decode_xml_entities(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("\n", " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_caption_tracks() {
        let player_json = json!({
            "captions": {
                "playerCaptionsTracklistRenderer": {
                    "captionTracks": [
                        {
                            "baseUrl": "https://www.youtube.com/api/timedtext?v=test&lang=en",
                            "name": { "simpleText": "English (auto-generated)" },
                            "languageCode": "en",
                            "kind": "asr",
                            "isTranslatable": true
                        },
                        {
                            "baseUrl": "https://www.youtube.com/api/timedtext?v=test&lang=id",
                            "name": { "simpleText": "Indonesian" },
                            "languageCode": "id",
                            "isTranslatable": true
                        }
                    ]
                }
            }
        });

        let tracks = extract_caption_tracks_from_player(&player_json).expect("Failed to extract caption tracks");
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].language_code, "en");
        assert_eq!(tracks[0].name, "English (auto-generated)");
        assert_eq!(tracks[0].kind.as_deref(), Some("asr"));
        assert_eq!(tracks[1].language_code, "id");
    }

    #[test]
    fn test_parse_json3_transcript() {
        let json3 = json!({
            "events": [
                {
                    "tStartMs": 1000,
                    "dDurationMs": 2500,
                    "segs": [
                        { "utf8": "Hello " },
                        { "utf8": "world!" }
                    ]
                },
                {
                    "tStartMs": 4000,
                    "dDurationMs": 3000,
                    "segs": [
                        { "utf8": "This is a subtitle test." }
                    ]
                }
            ]
        }).to_string();

        let transcript = parse_transcript_response("testVid", "en", &json3).expect("Failed to parse json3 transcript");
        assert_eq!(transcript.segments.len(), 2);
        assert_eq!(transcript.segments[0].start_ms, 1000);
        assert_eq!(transcript.segments[0].end_ms, 3500);
        assert_eq!(transcript.segments[0].text, "Hello world!");

        // Test SRT export
        let srt = transcript.to_srt();
        assert!(srt.contains("00:00:01,000 --> 00:00:03,500"));
        assert!(srt.contains("Hello world!"));

        // Test VTT export
        let vtt = transcript.to_vtt();
        assert!(vtt.starts_with("WEBVTT"));
        assert!(vtt.contains("00:00:01.000 --> 00:00:03.500"));
    }

    #[test]
    fn test_parse_xml_transcript() {
        let xml = r#"<transcript>
            <text start="1.5" dur="2.0">Testing XML &amp; subtitles</text>
        </transcript>"#;

        let transcript = parse_transcript_response("testVid", "en", xml).expect("Failed to parse xml transcript");
        assert_eq!(transcript.segments.len(), 1);
        assert_eq!(transcript.segments[0].start_ms, 1500);
        assert_eq!(transcript.segments[0].duration_ms, 2000);
        assert_eq!(transcript.segments[0].text, "Testing XML & subtitles");
    }
}
