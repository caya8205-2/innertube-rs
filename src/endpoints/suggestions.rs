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

// Re-export suggestion parsers from AST parser module
pub use crate::parser::nodes::suggestions::{parse_search_suggestions, parse_suggestions_payload};

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
