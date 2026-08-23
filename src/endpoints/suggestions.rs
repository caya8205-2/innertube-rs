use reqwest::header::HeaderMap;
use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::Result;
use crate::models::suggestions::SearchSuggestionsResult;

/// Fetch search autocomplete suggestions for YouTube or YouTube Music.
pub async fn get_search_suggestions(
    session: &Session,
    query: &str,
    is_music: bool,
) -> Result<SearchSuggestionsResult> {
    if is_music {
        // Use YouTube Music dedicated InnerTube endpoint
        let payload = json!({
            "input": query,
        });

        let resp: reqwest::Response = session
            .post_innertube_client("WEB_REMIX", "/music/get_search_suggestions", payload)
            .await?;
        let raw: Value = resp.json().await?;

        let mut suggestions = Vec::new();
        if let Some(contents) = raw.get("contents").and_then(|c| c.as_array()) {
            for sec in contents {
                if let Some(sug_sec) = sec.get("searchSuggestionsSectionRenderer") {
                    if let Some(items) = sug_sec.get("contents").and_then(|i| i.as_array()) {
                        for item in items {
                            if let Some(ssr) = item.get("searchSuggestionRenderer") {
                                if let Some(runs) = ssr.pointer("/suggestion/runs").and_then(|r| r.as_array()) {
                                    let full_text: String = runs
                                        .iter()
                                        .filter_map(|r| r.get("text").and_then(|t| t.as_str()))
                                        .collect();
                                    if !full_text.is_empty() {
                                        suggestions.push(full_text);
                                    }
                                }
                            } else if let Some(history) = item.get("historySuggestionRenderer") {
                                if let Some(runs) = history.pointer("/suggestion/runs").and_then(|r| r.as_array()) {
                                    let full_text: String = runs
                                        .iter()
                                        .filter_map(|r| r.get("text").and_then(|t| t.as_str()))
                                        .collect();
                                    if !full_text.is_empty() {
                                        suggestions.push(full_text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(SearchSuggestionsResult {
            query: query.to_string(),
            is_music: true,
            suggestions,
        })
    } else {
        // Standard YouTube suggestions
        let client = &session.http_client;
        let base_url = "https://suggestqueries-clients6.youtube.com/complete/search";

        let mut headers = HeaderMap::new();
        headers.insert(
            "Referer",
            "https://www.youtube.com/".parse().unwrap(),
        );

        let resp: reqwest::Response = client
            .get(base_url)
            .headers(headers)
            .query(&[
                ("client", "youtube"),
                ("ds", "yt"),
                ("q", query),
                ("hl", "en"),
                ("gl", "US"),
            ])
            .send()
            .await?;

        let body_text: String = resp.text().await?;
        let suggestions = parse_suggestions_payload(&body_text);

        Ok(SearchSuggestionsResult {
            query: query.to_string(),
            is_music: false,
            suggestions,
        })
    }
}

/// Parse Google suggestqueries response format: `["query", [["sug1", 0], ["sug2", 0]]]`.
pub fn parse_suggestions_payload(raw_text: &str) -> Vec<String> {
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

    if let Ok(val) = serde_json::from_str::<Value>(json_str) {
        if let Some(arr) = val.as_array() {
            if arr.len() >= 2 {
                if let Some(sug_list) = arr[1].as_array() {
                    for sug in sug_list {
                        if let Some(inner_arr) = sug.as_array() {
                            if let Some(text) = inner_arr.first().and_then(|t| t.as_str()) {
                                results.push(text.to_string());
                            }
                        } else if let Some(text) = sug.as_str() {
                            results.push(text.to_string());
                        }
                    }
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_suggestions_json() {
        let sample = r#"["yoasobi", [["yoasobi idol", 0, [0]], ["yoasobi monster", 0, [0]], ["yoasobi playlist", 0, [0]]]]"#;
        let suggestions = parse_suggestions_payload(sample);
        assert_eq!(suggestions.len(), 3);
        assert_eq!(suggestions[0], "yoasobi idol");
        assert_eq!(suggestions[1], "yoasobi monster");
        assert_eq!(suggestions[2], "yoasobi playlist");
    }

    #[test]
    fn test_parse_suggestions_jsonp() {
        let sample = r#"window.google.ac.h(["alan walker", [["alan walker faded", 0], ["alan walker alone", 0]]])"#;
        let suggestions = parse_suggestions_payload(sample);
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0], "alan walker faded");
        assert_eq!(suggestions[1], "alan walker alone");
    }
}
