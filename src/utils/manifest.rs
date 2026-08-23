use crate::error::{InnertubeError, Result};
use crate::models::manifest::ManifestStream;

/// Parse an HLS Master Playlist (.m3u8) string into a list of stream variants.
pub fn parse_hls_manifest(m3u8_content: &str) -> Result<Vec<ManifestStream>> {
    let mut streams = Vec::new();
    let lines: Vec<&str> = m3u8_content.lines().map(|l| l.trim()).collect();

    let mut current_bandwidth = None;
    let mut current_resolution = None;
    let mut current_codecs = None;
    let mut current_frame_rate = None;

    for line in lines {
        if let Some(tags) = line.strip_prefix("#EXT-X-STREAM-INF:") {
            current_bandwidth = extract_hls_attr(tags, "BANDWIDTH").and_then(|b| b.parse::<u64>().ok());
            current_resolution = extract_hls_attr(tags, "RESOLUTION");
            current_codecs = extract_hls_attr(tags, "CODECS").map(|s| s.to_string());
            current_frame_rate = extract_hls_attr(tags, "FRAME-RATE").and_then(|f| f.parse::<f32>().ok());
        } else if !line.starts_with('#') && !line.is_empty() {
            // This is the stream URI
            let url = line.to_string();
            let mut width = None;
            let mut height = None;

            if let Some(res) = &current_resolution {
                let parts: Vec<&str> = res.split('x').collect();
                if parts.len() == 2 {
                    width = parts[0].parse::<u32>().ok();
                    height = parts[1].parse::<u32>().ok();
                }
            }

            let itag = extract_itag_from_url(&url);
            let mime_type = if height.is_some() || width.is_some() {
                "video/mp4".to_string()
            } else {
                "audio/mp4".to_string()
            };

            streams.push(ManifestStream {
                itag,
                mime_type,
                codecs: current_codecs.take(),
                bandwidth: current_bandwidth.take(),
                width,
                height,
                frame_rate: current_frame_rate.take(),
                audio_channels: None,
                sample_rate: None,
                url,
                is_live: true,
            });

            current_resolution = None;
        }
    }

    Ok(streams)
}

/// Parse a DASH MPD (.mpd) XML string into a list of stream representations.
pub fn parse_dash_manifest(mpd_xml: &str) -> Result<Vec<ManifestStream>> {
    let mut streams = Vec::new();

    // Split by AdaptationSet
    for adaptation in mpd_xml.split("<AdaptationSet") {
        let mime_type = extract_xml_attr(adaptation, "mimeType").unwrap_or("video/mp4").to_string();
        let adapt_codecs = extract_xml_attr(adaptation, "codecs").map(|s| s.to_string());

        for rep in adaptation.split("<Representation") {
            if !rep.contains("id=") && !rep.contains("bandwidth=") {
                continue;
            }

            let itag = extract_xml_attr(rep, "id").and_then(|s| s.parse::<u32>().ok());
            let bandwidth = extract_xml_attr(rep, "bandwidth").and_then(|s| s.parse::<u64>().ok());
            let width = extract_xml_attr(rep, "width").and_then(|s| s.parse::<u32>().ok());
            let height = extract_xml_attr(rep, "height").and_then(|s| s.parse::<u32>().ok());
            let frame_rate = extract_xml_attr(rep, "frameRate").and_then(|s| s.parse::<f32>().ok());
            let codecs = extract_xml_attr(rep, "codecs").map(|s| s.to_string()).or_else(|| adapt_codecs.clone());

            let mut url = String::new();
            if let Some(base_url_start) = rep.find("<BaseURL>") {
                let start = base_url_start + "<BaseURL>".len();
                if let Some(end) = rep[start..].find("</BaseURL>") {
                    url = rep[start..start + end].trim().to_string();
                }
            }

            if !url.is_empty() {
                streams.push(ManifestStream {
                    itag,
                    mime_type: mime_type.clone(),
                    codecs,
                    bandwidth,
                    width,
                    height,
                    frame_rate,
                    audio_channels: None,
                    sample_rate: None,
                    url,
                    is_live: false,
                });
            }
        }
    }

    Ok(streams)
}

/// Fetch an HLS manifest URL and parse its streams.
pub async fn fetch_and_parse_hls(http_client: &reqwest::Client, hls_url: &str) -> Result<Vec<ManifestStream>> {
    let resp = http_client.get(hls_url).send().await.map_err(InnertubeError::Network)?;
    let text = resp.text().await.map_err(InnertubeError::Network)?;
    parse_hls_manifest(&text)
}

/// Fetch a DASH manifest URL and parse its representations.
pub async fn fetch_and_parse_dash(http_client: &reqwest::Client, dash_url: &str) -> Result<Vec<ManifestStream>> {
    let resp = http_client.get(dash_url).send().await.map_err(InnertubeError::Network)?;
    let text = resp.text().await.map_err(InnertubeError::Network)?;
    parse_dash_manifest(&text)
}

fn extract_hls_attr<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pattern = format!("{}=", key);
    let start = line.find(&pattern)? + pattern.len();
    let rest = &line[start..];

    if let Some(stripped) = rest.strip_prefix('"') {
        let end = stripped.find('"')?;
        Some(&stripped[..end])
    } else {
        let end = rest.find(',').unwrap_or(rest.len());
        Some(rest[..end].trim())
    }
}

fn extract_xml_attr<'a>(chunk: &'a str, attr_name: &str) -> Option<&'a str> {
    let patterns = [
        format!(" {}=", attr_name),
        format!("\n{}=", attr_name),
        format!("\t{}=", attr_name),
        format!("<{}=", attr_name),
    ];

    for pat in patterns {
        if let Some(pos) = chunk.find(&pat) {
            let after_eq = &chunk[pos + pat.len()..];
            if let Some(stripped) = after_eq.strip_prefix('"') {
                if let Some(end_quote) = stripped.find('"') {
                    return Some(&stripped[..end_quote]);
                }
            } else if let Some(stripped) = after_eq.strip_prefix('\'') {
                if let Some(end_quote) = stripped.find('\'') {
                    return Some(&stripped[..end_quote]);
                }
            } else {
                let end_space = after_eq.find(|c: char| c.is_whitespace() || c == '>' || c == '/').unwrap_or(after_eq.len());
                return Some(&after_eq[..end_space]);
            }
        }
    }
    None
}

fn extract_itag_from_url(url: &str) -> Option<u32> {
    if let Some(pos) = url.find("/itag/") {
        let rest = &url[pos + 6..];
        let end = rest.find('/').unwrap_or_else(|| rest.find('?').unwrap_or(rest.len()));
        return rest[..end].parse::<u32>().ok();
    }
    if let Some(pos) = url.find("itag=") {
        let rest = &url[pos + 5..];
        let end = rest.find('&').unwrap_or(rest.len());
        return rest[..end].parse::<u32>().ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hls_manifest() {
        let fixture = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:BANDWIDTH=1500000,RESOLUTION=1280x720,FRAME-RATE=30.000,CODECS="avc1.4d401f,mp4a.40.2"
https://manifest.googlevideo.com/api/manifest/hls_variant/itag/96/file/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=800000,RESOLUTION=854x480,FRAME-RATE=30.000,CODECS="avc1.4d401e,mp4a.40.2"
https://manifest.googlevideo.com/api/manifest/hls_variant/itag/95/file/index.m3u8
"#;

        let streams = parse_hls_manifest(fixture).expect("Failed to parse HLS");
        assert_eq!(streams.len(), 2);
        assert_eq!(streams[0].itag, Some(96));
        assert_eq!(streams[0].bandwidth, Some(1500000));
        assert_eq!(streams[0].width, Some(1280));
        assert_eq!(streams[0].height, Some(720));
        assert_eq!(streams[0].frame_rate, Some(30.0));
        assert_eq!(streams[0].codecs.as_deref(), Some("avc1.4d401f,mp4a.40.2"));
        assert_eq!(streams[1].itag, Some(95));
        assert_eq!(streams[1].height, Some(480));
    }

    #[test]
    fn test_parse_dash_manifest() {
        let fixture = r#"<MPD>
            <Period>
                <AdaptationSet mimeType="video/mp4" codecs="avc1.640028">
                    <Representation id="137" bandwidth="4500000" width="1920" height="1080" frameRate="30">
                        <BaseURL>https://rr1.googlevideo.com/videoplayback?itag=137</BaseURL>
                    </Representation>
                </AdaptationSet>
            </Period>
        </MPD>"#;

        let streams = parse_dash_manifest(fixture).expect("Failed to parse DASH");
        assert_eq!(streams.len(), 1);
        assert_eq!(streams[0].itag, Some(137));
        assert_eq!(streams[0].bandwidth, Some(4500000));
        assert_eq!(streams[0].width, Some(1920));
        assert_eq!(streams[0].height, Some(1080));
        assert_eq!(streams[0].url, "https://rr1.googlevideo.com/videoplayback?itag=137");
    }
}
