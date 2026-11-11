use serde_json::Value;

use crate::models::suggestions::SearchSuggestion;

/// Parse Google suggestqueries response format into structured `SearchSuggestion` items.
pub fn parse_search_suggestions(raw_text: &str) -> Vec<SearchSuggestion> {
    let mut results = Vec::new();

    // Sometimes responses have jsonp wrappers like `window.google.ac.h(...)`
    let json_str = if let Some(start) = raw_text.find('(') {
        if let Some(end) = raw_text.rfind(')') {
            &raw_text[start + 1..end]
        } else {
            raw_text
        }
    } else {
        raw_text
    };

    let Ok(val) = serde_json::from_str::<Value>(json_str) else {
        return results;
    };

    let Some(sug_list) = val.pointer("/1").and_then(|v| v.as_array()) else {
        return results;
    };

    for sug in sug_list {
        let text = sug.pointer("/0")
            .and_then(|t| t.as_str())
            .or_else(|| sug.as_str());

        let Some(text) = text else {
            continue;
        };

        // Index 2: flags array (flag 39 indicates search history)
        let is_history = sug.pointer("/2")
            .and_then(|f| f.as_array())
            .map(|flags| flags.iter().any(|flag| flag.as_i64() == Some(39)))
            .unwrap_or(false);

        // Index 3: rich metadata object
        let subtitle = sug.pointer("/3/zaf")
            .or_else(|| sug.pointer("/3/subtitle"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let thumbnail = sug.pointer("/3/zai")
            .or_else(|| sug.pointer("/3/thumbnail"))
            .and_then(|t| t.as_str())
            .map(|thumb| {
                if thumb.starts_with("//") {
                    format!("https:{thumb}")
                } else {
                    thumb.to_string()
                }
            });

        let channel_id = sug.pointer("/3/zav")
            .or_else(|| sug.pointer("/3/channelId"))
            .and_then(|c| c.as_str())
            .map(|c| c.to_string());

        let canonical_name = sug.pointer("/3/zao")
            .or_else(|| sug.pointer("/3/canonicalName"))
            .and_then(|n| n.as_str())
            .map(|n| n.to_string());

        let entity_id = sug.pointer("/3/zae")
            .or_else(|| sug.pointer("/3/entityId"))
            .and_then(|e| e.as_str())
            .map(|e| e.to_string());

        let info_url = sug.pointer("/3/zaq")
            .or_else(|| sug.pointer("/3/infoUrl"))
            .and_then(|u| u.as_str())
            .map(|u| u.to_string());

        results.push(SearchSuggestion {
            text: text.to_string(),
            subtitle,
            thumbnail,
            channel_id,
            canonical_name,
            entity_id,
            info_url,
            is_history,
        });
    }

    results
}

/// Parse Google suggestqueries response format: `["query", [["sug1", 0], ["sug2", 0]]]`.
pub fn parse_suggestions_payload(raw_text: &str) -> Vec<String> {
    parse_search_suggestions(raw_text)
        .into_iter()
        .map(|s| s.text)
        .collect()
}

