use crate::utils::streaming_info::{SegmentInfo, StreamingInfo};

/// Escape XML text/attribute content (legacy DashUtils render rules).
fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn drm_system_id(drm_family: &str) -> Option<&'static str> {
    match drm_family {
        "WIDEVINE" => Some("edef8ba9-79d6-4ace-a3c8-27dcd51d21ed"),
        "PLAYREADY" => Some("9a04f079-9840-4286-ab92-e65be0885f95"),
        _ => None,
    }
}

/// Format seconds the way JS stringifies numbers (`PT213S`, `PT213.5S`).
fn format_duration_secs(secs: f64) -> String {
    if secs.fract() == 0.0 {
        format!("{}", secs as u64)
    } else {
        format!("{secs}")
    }
}

fn render_segment_info(info: &SegmentInfo, out: &mut String) {
    match info {
        SegmentInfo::Base {
            base_url,
            index_range,
            init_range,
        } => {
            out.push_str(&format!(
                "<BaseURL>{}</BaseURL>",
                escape_xml(base_url)
            ));
            out.push_str(&format!(
                "<SegmentBase indexRange=\"{}-{}\">",
                index_range.start, index_range.end
            ));
            out.push_str(&format!(
                "<Initialization range=\"{}-{}\"/>",
                init_range.start, init_range.end
            ));
            out.push_str("</SegmentBase>");
        }
        SegmentInfo::Template(template) => {
            out.push_str(&format!(
                "<SegmentTemplate startNumber=\"{}\" timescale=\"1000\"",
                if template.init_url.is_some() { "1" } else { "0" }
            ));
            if let Some(ref init) = template.init_url {
                out.push_str(&format!(" initialization=\"{}\"", escape_xml(init)));
            }
            out.push_str(&format!(" media=\"{}\">", escape_xml(&template.media_url)));
            out.push_str("<SegmentTimeline>");
            for segment in &template.timeline {
                match segment.repeat_count {
                    Some(r) => out.push_str(&format!("<S d=\"{}\" r=\"{r}\"/>", segment.duration)),
                    None => out.push_str(&format!("<S d=\"{}\"/>", segment.duration)),
                }
            }
            out.push_str("</SegmentTimeline></SegmentTemplate>");
        }
    }
}

/// Render the DASH MPD manifest (legacy `DashManifest`), matching legacy
/// element/attribute layout and escaping rules.
pub fn render_dash_manifest(info: &StreamingInfo) -> String {
    let mut out = String::with_capacity(4096);
    out.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
    out.push_str(&format!(
        "<MPD xmlns=\"urn:mpeg:dash:schema:mpd:2011\" minBufferTime=\"PT1.500S\" profiles=\"urn:mpeg:dash:profile:isoff-main:2011\" type=\"static\" mediaPresentationDuration=\"PT{}S\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"urn:mpeg:dash:schema:mpd:2011 http://standards.iso.org/ittf/PubliclyAvailableStandards/MPEG-DASH_schema_files/DASH-MPD.xsd\">",
        format_duration_secs(info.duration_secs)
    ));
    out.push_str("<Period>");

    for (index, set) in info.audio_sets.iter().enumerate() {
        out.push_str(&format!(
            "<AdaptationSet id=\"{index}\" mimeType=\"{}\" startWithSAP=\"1\" subsegmentAlignment=\"true\"",
            set.mime_type
        ));
        if let Some(ref lang) = set.language {
            out.push_str(&format!(" lang=\"{}\"", escape_xml(lang)));
        }
        if let Some(ref codecs) = set.codecs {
            out.push_str(&format!(" codecs=\"{}\"", escape_xml(codecs)));
        }
        if let Some(rate) = set.audio_sample_rate {
            out.push_str(&format!(" audioSamplingRate=\"{rate}\""));
        }
        out.push_str(" contentType=\"audio\">");

        if let Some(ref families) = set.drm_families {
            for family in families {
                if let Some(id) = drm_system_id(family) {
                    out.push_str(&format!(
                        "<ContentProtection schemeIdUri=\"urn:uuid:{id}\"/>"
                    ));
                }
            }
        }
        for role in &set.track_roles {
            out.push_str(&format!(
                "<Role schemeIdUri=\"urn:mpeg:dash:role:2011\" value=\"{}\"/>",
                escape_xml(role)
            ));
        }
        if let Some(ref name) = set.track_name {
            out.push_str(&format!("<Label id=\"{index}\">{}</Label>", escape_xml(name)));
        }
        if let Some(channels) = set.channels {
            out.push_str(&format!(
                "<AudioChannelConfiguration schemeIdUri=\"urn:mpeg:dash:23003:3:audio_channel_configuration:2011\" value=\"{channels}\"/>"
            ));
        }

        for rep in &set.representations {
            out.push_str(&format!(
                "<Representation id=\"{}\" bandwidth=\"{}\"",
                escape_xml(&rep.uid),
                rep.bitrate
            ));
            if let Some(ref codecs) = rep.codecs {
                out.push_str(&format!(" codecs=\"{}\"", escape_xml(codecs)));
            }
            if let Some(rate) = rep.audio_sample_rate {
                out.push_str(&format!(" audioSamplingRate=\"{rate}\""));
            }
            out.push('>');
            if let Some(channels) = rep.channels {
                out.push_str(&format!(
                    "<AudioChannelConfiguration schemeIdUri=\"urn:mpeg:dash:23003:3:audio_channel_configuration:2011\" value=\"{channels}\"/>"
                ));
            }
            render_segment_info(&rep.segment_info, &mut out);
            out.push_str("</Representation>");
        }
        out.push_str("</AdaptationSet>");
    }

    for (index, set) in info.video_sets.iter().enumerate() {
        out.push_str(&format!(
            "<AdaptationSet id=\"{}\" mimeType=\"{}\" startWithSAP=\"1\" subsegmentAlignment=\"true\"",
            index + info.audio_sets.len(),
            set.mime_type
        ));
        if let Some(ref codecs) = set.codecs {
            out.push_str(&format!(" codecs=\"{}\"", escape_xml(codecs)));
        }
        out.push_str(" maxPlayoutRate=\"1\"");
        if let Some(fps) = set.fps {
            out.push_str(&format!(" frameRate=\"{fps}\""));
        }
        out.push_str(" contentType=\"video\">");

        if let Some(ref families) = set.drm_families {
            for family in families {
                if let Some(id) = drm_system_id(family) {
                    out.push_str(&format!(
                        "<ContentProtection schemeIdUri=\"urn:uuid:{id}\"/>"
                    ));
                }
            }
        }
        if let Some(p) = set.color_info.primaries {
            out.push_str(&format!(
                "<SupplementalProperty schemeIdUri=\"urn:mpeg:mpegB:cicp:ColourPrimaries\" value=\"{p}\"/>"
            ));
        }
        if let Some(t) = set.color_info.transfer_characteristics {
            out.push_str(&format!(
                "<SupplementalProperty schemeIdUri=\"urn:mpeg:mpegB:cicp:TransferCharacteristics\" value=\"{t}\"/>"
            ));
        }
        if let Some(m) = set.color_info.matrix_coefficients {
            out.push_str(&format!(
                "<SupplementalProperty schemeIdUri=\"urn:mpeg:mpegB:cicp:MatrixCoefficients\" value=\"{m}\"/>"
            ));
        }

        for rep in &set.representations {
            out.push_str(&format!(
                "<Representation id=\"{}\" bandwidth=\"{}\"",
                escape_xml(&rep.uid),
                rep.bitrate
            ));
            if let Some(w) = rep.width {
                out.push_str(&format!(" width=\"{w}\""));
            }
            if let Some(h) = rep.height {
                out.push_str(&format!(" height=\"{h}\""));
            }
            if let Some(ref codecs) = rep.codecs {
                out.push_str(&format!(" codecs=\"{}\"", escape_xml(codecs)));
            }
            if let Some(fps) = rep.fps {
                out.push_str(&format!(" frameRate=\"{fps}\""));
            }
            out.push('>');
            render_segment_info(&rep.segment_info, &mut out);
            out.push_str("</Representation>");
        }
        out.push_str("</AdaptationSet>");
    }

    for (index, set) in info.image_sets.iter().enumerate() {
        out.push_str(&format!(
            "<AdaptationSet id=\"{}\" mimeType=\"{}\" contentType=\"image\">",
            index + info.audio_sets.len() + info.video_sets.len(),
            set.mime_type
        ));
        for rep in &set.representations {
            out.push_str(&format!(
                "<Representation id=\"thumbnails_{}x{}\" bandwidth=\"{}\" width=\"{}\" height=\"{}\">",
                rep.thumbnail_width,
                rep.thumbnail_height,
                rep.bitrate.unwrap_or(0),
                rep.sheet_width,
                rep.sheet_height
            ));
            out.push_str(&format!(
                "<EssentialProperty schemeIdUri=\"http://dashif.org/thumbnail_tile\" value=\"{}x{}\"/>",
                rep.columns, rep.rows
            ));
            out.push_str(&format!(
                "<SegmentTemplate media=\"{}\" duration=\"{}\" startNumber=\"0\"/>",
                escape_xml(&rep.template_url),
                rep.template_duration
            ));
            out.push_str("</Representation>");
        }
        out.push_str("</AdaptationSet>");
    }

    for (index, set) in info.text_sets.iter().enumerate() {
        out.push_str(&format!(
            "<AdaptationSet id=\"{}\" mimeType=\"{}\" lang=\"{}\" contentType=\"text\">",
            index + info.audio_sets.len() + info.video_sets.len() + info.image_sets.len(),
            set.mime_type,
            escape_xml(&set.language)
        ));
        for role in &set.track_roles {
            out.push_str(&format!(
                "<Role schemeIdUri=\"urn:mpeg:dash:role:2011\" value=\"{}\"/>",
                escape_xml(role)
            ));
        }
        out.push_str(&format!(
            "<Label id=\"{}\">{}</Label>",
            index + info.audio_sets.len(),
            escape_xml(&set.track_name)
        ));
        out.push_str(&format!(
            "<Representation id=\"{}\" bandwidth=\"0\"><BaseURL>{}</BaseURL></Representation>",
            escape_xml(&set.representation.uid),
            escape_xml(&set.representation.base_url)
        ));
        out.push_str("</AdaptationSet>");
    }

    out.push_str("</Period></MPD>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::format::FormatRange;
    use crate::utils::streaming_info::{
        AudioRepresentation, AudioSet, SegmentInfo, StreamingInfo,
    };

    fn base_info() -> StreamingInfo {
        StreamingInfo {
            duration_secs: 213.0,
            audio_sets: vec![AudioSet {
                mime_type: "audio/mp4".to_string(),
                language: Some("en".to_string()),
                codecs: None,
                audio_sample_rate: None,
                track_name: None,
                track_roles: vec!["main".to_string()],
                channels: None,
                drm_families: None,
                drm_track_type: None,
                representations: vec![AudioRepresentation {
                    uid: "140".to_string(),
                    bitrate: 128_000,
                    codecs: Some("mp4a.40.2".to_string()),
                    audio_sample_rate: Some(44100),
                    channels: Some(2),
                    segment_info: SegmentInfo::Base {
                        base_url: "https://googlevideo.com/v?itag=140".to_string(),
                        index_range: FormatRange {
                            start: "592".to_string(),
                            end: "1000".to_string(),
                        },
                        init_range: FormatRange {
                            start: "0".to_string(),
                            end: "591".to_string(),
                        },
                    },
                }],
            }],
            video_sets: vec![],
            image_sets: vec![],
            text_sets: vec![],
        }
    }

    #[test]
    fn dash_manifest_structure_matches_legacy() {
        let xml = render_dash_manifest(&base_info());
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
        assert!(xml.contains("<MPD xmlns=\"urn:mpeg:dash:schema:mpd:2011\""));
        assert!(xml.contains("minBufferTime=\"PT1.500S\""));
        assert!(xml.contains("mediaPresentationDuration=\"PT213S\""));
        assert!(xml.contains(
            "<AdaptationSet id=\"0\" mimeType=\"audio/mp4\" startWithSAP=\"1\" subsegmentAlignment=\"true\" lang=\"en\" contentType=\"audio\">"
        ));
        assert!(xml.contains("<Role schemeIdUri=\"urn:mpeg:dash:role:2011\" value=\"main\"/>"));
        assert!(xml.contains(
            "<Representation id=\"140\" bandwidth=\"128000\" codecs=\"mp4a.40.2\" audioSamplingRate=\"44100\">"
        ));
        assert!(xml.contains("<BaseURL>https://googlevideo.com/v?itag=140</BaseURL>"));
        assert!(xml.contains(
            "<SegmentBase indexRange=\"592-1000\"><Initialization range=\"0-591\"/></SegmentBase>"
        ));
        assert!(xml.ends_with("</Period></MPD>"));
    }

    #[test]
    fn dash_escapes_ampersand_in_urls() {
        let mut info = base_info();
        if let SegmentInfo::Base { base_url, .. } =
            &mut info.audio_sets[0].representations[0].segment_info
        {
            *base_url = "https://googlevideo.com/v?a=1&b=2".to_string();
        }
        let xml = render_dash_manifest(&info);
        assert!(xml.contains("a=1&amp;b=2"), "{xml}");
    }

    #[test]
    fn dash_segment_template_with_timeline() {
        let mut info = base_info();
        info.audio_sets[0].representations[0].segment_info =
            SegmentInfo::Template(crate::utils::streaming_info::SegmentTemplate {
                init_url: Some("https://googlevideo.com/v&sq=0".to_string()),
                media_url: "https://googlevideo.com/v&sq=$Number$".to_string(),
                timeline: vec![
                    crate::utils::streaming_info::Segment {
                        duration: 5120,
                        repeat_count: Some(920),
                    },
                    crate::utils::streaming_info::Segment {
                        duration: 3600,
                        repeat_count: None,
                    },
                ],
            });
        let xml = render_dash_manifest(&info);
        assert!(xml.contains(
            "<SegmentTemplate startNumber=\"1\" timescale=\"1000\" initialization=\"https://googlevideo.com/v&amp;sq=0\" media=\"https://googlevideo.com/v&amp;sq=$Number$\">"
        ));
        assert!(xml.contains("<S d=\"5120\" r=\"920\"/><S d=\"3600\"/>"));
    }

    #[test]
    fn dash_fractional_duration() {
        let mut info = base_info();
        info.duration_secs = 213.5;
        let xml = render_dash_manifest(&info);
        assert!(xml.contains("mediaPresentationDuration=\"PT213.5S\""));
    }
}
