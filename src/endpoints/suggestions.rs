use reqwest::header::HeaderMap;
use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::Result;
use crate::models::suggestions::{SearchSuggestion, SearchSuggestionsResult};

/// Fetch search autocomplete suggestions for YouTube or YouTube Music with optional previous_query.
pub async fn get_search_suggestions_with_options(
    session: &Session,
    query: &str,
    previous_query: Option<&str>,
    is_music: bool,
) -> Result<SearchSuggestionsResult> {
    if is_music {
        // Use YouTube Music dedicated InnerTube endpoint
        let payload = json!({
            "input": query,
        });

        let resp: reqwest::Response = session
            .post_innertube_client("YTMUSIC", "/music/get_search_suggestions", payload)
            .await?;
        let raw: Value = resp.json().await?;

        let mut suggestions = Vec::new();
        let mut items = Vec::new();

        if let Some(contents) = raw.pointer("/contents").and_then(|c| c.as_array()) {
            for sec in contents {
                if let Some(item_arr) = sec.pointer("/searchSuggestionsSectionRenderer/contents").and_then(|i| i.as_array()) {
                    for item in item_arr {
                        let is_history = item.pointer("/historySuggestionRenderer").is_some();
                        let runs = item.pointer("/searchSuggestionRenderer/suggestion/runs")
                            .or_else(|| item.pointer("/historySuggestionRenderer/suggestion/runs"))
                            .and_then(|r| r.as_array());

                        if let Some(runs) = runs {
                            let full_text: String = runs
                                .iter()
                                .filter_map(|r| r.pointer("/text").and_then(|t| t.as_str()))
                                .collect();

                            if !full_text.is_empty() {
                                suggestions.push(full_text.clone());
                                items.push(SearchSuggestion {
                                    text: full_text,
                                    subtitle: None,
                                    thumbnail: None,
                                    channel_id: None,
                                    canonical_name: None,
                                    entity_id: None,
                                    info_url: None,
                                    is_history,
                                });
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
            items,
        })
    } else {
        // Standard YouTube suggestions
        let client = &session.http_client;
        let base_url = "https://suggestqueries-clients6.youtube.com/complete/search";

        let mut headers = HeaderMap::new();
        headers.insert("Referer", "https://www.youtube.com/".parse().unwrap());
        if let Some(ref cookie) = session.cookie {
            if let Ok(c_val) = cookie.parse() {
                headers.insert("Cookie", c_val);
            }
        }

        let mut query_params = vec![
            ("client", "youtube".to_string()),
            ("gs_ri", "youtube".to_string()),
            ("gs_id", "0".to_string()),
            ("cp", "0".to_string()),
            ("ds", "yt".to_string()),
            ("sugexp", "yt-control-0".to_string()),
            ("hl", session.context.client.hl.clone()),
            ("gl", session.context.client.gl.clone()),
            ("q", query.to_string()),
        ];

        if let Some(pq) = previous_query {
            query_params.push(("pq", pq.to_string()));
        }

        let resp: reqwest::Response = client
            .get(base_url)
            .headers(headers)
            .query(&query_params)
            .send()
            .await?;

        let body_text: String = resp.text().await?;
        let items = parse_search_suggestions(&body_text);
        let suggestions = items.iter().map(|it| it.text.clone()).collect();

        Ok(SearchSuggestionsResult {
            query: query.to_string(),
            is_music: false,
            suggestions,
            items,
        })
    }
}

/// Fetch search autocomplete suggestions for YouTube or YouTube Music.
pub async fn get_search_suggestions(
    session: &Session,
    query: &str,
    is_music: bool,
) -> Result<SearchSuggestionsResult> {
    get_search_suggestions_with_options(session, query, None, is_music).await
}

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

    #[test]
    fn test_parse_suggestions_rich_metadata() {
        let sample = r#"window.google.ac.h(["mrbeast",[["mrbeast",0,[512,433,790,39],{"zae":"/g/11ffmnm29k","zaf":"@MrBeast","zai":"//yt3.googleusercontent.com/test.jpg","zao":"MrBeast","zav":"UCX6OQ3DkcsbYNE6H8uQQuVA","zaq":"https://en.wikipedia.org/wiki/MrBeast"}],["mrbeast gaming",0,[512,433]]]])"#;
        let items = parse_search_suggestions(sample);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].text, "mrbeast");
        assert_eq!(items[0].subtitle.as_deref(), Some("@MrBeast"));
        assert_eq!(items[0].thumbnail.as_deref(), Some("https://yt3.googleusercontent.com/test.jpg"));
        assert_eq!(items[0].channel_id.as_deref(), Some("UCX6OQ3DkcsbYNE6H8uQQuVA"));
        assert_eq!(items[0].canonical_name.as_deref(), Some("MrBeast"));
        assert_eq!(items[0].entity_id.as_deref(), Some("/g/11ffmnm29k"));
        assert_eq!(items[0].info_url.as_deref(), Some("https://en.wikipedia.org/wiki/MrBeast"));
        assert!(items[0].is_history);

        assert_eq!(items[1].text, "mrbeast gaming");
        assert_eq!(items[1].subtitle, None);
        assert_eq!(items[1].thumbnail, None);
        assert_eq!(items[1].channel_id, None);
        assert!(!items[1].is_history);
    }
}
