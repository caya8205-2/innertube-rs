use regex::Regex;
use rquickjs::{Context, Function, Object, Runtime};
use std::collections::HashMap;
use crate::constants::clients;
use crate::error::{InnertubeError, Result};

pub struct DecipherResult {
    pub sig: Option<String>,
    pub n: Option<String>,
}

/// Per-player-response n-token cache (`this_response_nsig_cache` in
/// YouTube.js): deduplicates n-transform evaluation across formats of one
/// response.
pub type NsigCache = HashMap<String, String>;

/// Error sentinel prefix returned by YouTube's n-transform on failure.
/// Results with this prefix are used but never cached (legacy behavior).
pub const NSIG_ERROR_SENTINEL: &str = "enhanced_except_";

/// Decipher engine executing YouTube player decipher routines in QuickJS.
pub struct PlayerDecipherer {
    _runtime: Runtime,
    context: Context,
    pub nsig_fn_name: String,
    /// `signatureTimestamp` extracted from the player script. `None` when
    /// extraction fails (legacy logs a warning and continues without it).
    pub signature_timestamp: Option<u32>,
}

impl PlayerDecipherer {
    pub fn new(player_js: &str) -> Result<Self> {
        // 1. Extract signatureTimestamp
        let sts_re = Regex::new(r"signatureTimestamp\s*:\s*(\d+)")
            .map_err(|e| InnertubeError::Player(e.to_string()))?;

        let sts = sts_re
            .captures(player_js)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok());

        // 2. Extract nsig function name
        let fn_re = Regex::new(
            r#"(?:var\s+)?([a-zA-Z0-9_$]+)\s*=\s*function\s*\(\s*([a-zA-Z0-9_$]+)\s*,\s*([a-zA-Z0-9_$]+)\s*=\s*[^,]+\s*,\s*([a-zA-Z0-9_$]+)\s*=\s*[^)]+\)\s*\{[^}]*?\.set\(\s*["']alr["']\s*,\s*["']yes["']\s*\)"#,
        )
        .map_err(|e| InnertubeError::Player(e.to_string()))?;

        let nsig_fn_name = fn_re
            .captures(player_js)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| InnertubeError::Player("Failed to find nsig function name in base.js".into()))?;

        // 3. Initialize QuickJS runtime and context
        let runtime = Runtime::new().map_err(|e| InnertubeError::Player(e.to_string()))?;
        let context = Context::full(&runtime).map_err(|e| InnertubeError::Player(e.to_string()))?;

        // Export hook to capture the closure-scoped nsig function onto window.__nsig_fn
        let export_hook = format!("window.__nsig_fn = {nsig_fn_name}; }})(_yt_player);");
        let sanitized_js = player_js
            .replace("var window=this;", "var window=globalThis;")
            .replace("})(_yt_player);", &export_hook);

        context.with(|ctx| -> Result<()> {
            let global = ctx.globals();
            global.set("window", global.clone()).map_err(|e| InnertubeError::Player(e.to_string()))?;
            global.set("self", global.clone()).map_err(|e| InnertubeError::Player(e.to_string()))?;
            global.set("global", global.clone()).map_err(|e| InnertubeError::Player(e.to_string()))?;

            // Environment polyfills & browser shims for base.js
            ctx.eval::<(), _>(
                r#"
                window.document = {
                    createElement: function() { return {}; },
                    getElementById: function() { return null; },
                    getElementsByTagName: function() { return []; },
                    querySelector: function() { return null; },
                    querySelectorAll: function() { return []; },
                    addEventListener: function() {},
                    removeEventListener: function() {},
                    documentElement: {},
                    body: {},
                    currentScript: { src: "https://www.youtube.com/s/player/base.js" }
                };
                window.navigator = {
                    userAgent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36",
                    platform: "Win32"
                };
                window.location = {
                    href: "https://www.youtube.com",
                    protocol: "https:",
                    host: "www.youtube.com",
                    hostname: "www.youtube.com",
                    pathname: "/"
                };
                window.screen = { width: 1920, height: 1080 };
                window.history = {};
                window.XMLHttpRequest = function() {
                    return {
                        open: function() {},
                        send: function() {},
                        setRequestHeader: function() {},
                        addEventListener: function() {}
                    };
                };
                window.fetch = function() {
                    return Promise.resolve({
                        ok: true,
                        json: function() { return Promise.resolve({}); },
                        text: function() { return Promise.resolve(""); }
                    });
                };
                window.CustomEvent = function() {};
                window.Event = function() {};
                window.MessageChannel = function() { return { port1: {}, port2: {} }; };
                window.Intl = {
                    DateTimeFormat: Object.assign(function() { return { format: function() { return ""; } }; }, {
                        supportedLocalesOf: function(locales) { return locales || ["en"]; }
                    }),
                    NumberFormat: Object.assign(function() { return { format: function() { return ""; } }; }, {
                        supportedLocalesOf: function(locales) { return locales || ["en"]; }
                    }),
                    Collator: Object.assign(function() { return { compare: function() { return 0; } }; }, {
                        supportedLocalesOf: function(locales) { return locales || ["en"]; }
                    }),
                    Segmenter: Object.assign(function() { return { segment: function() { return []; } }; }, {
                        supportedLocalesOf: function(locales) { return locales || ["en"]; }
                    })
                };
            "#,
            )
            .map_err(|e| InnertubeError::Player(format!("Failed to setup JS sandbox: {e}")))?;

            if let Err(e) = ctx.eval::<(), _>(sanitized_js) {
                let exc = ctx.catch();
                return Err(InnertubeError::Player(format!(
                    "Failed to eval player_js: {e} (JS Exception: {exc:?})"
                )));
            }

            ctx.eval::<(), _>(
                r#"
                function __process_nsig(fnRef, n, sp, s) {
                    var mockUrl = "https://ytjs.googlevideo.com/videoplayback?expire=1234567890&n=" + encodeURIComponent(n || "");
                    var urlCtor = fnRef(mockUrl, sp || "", s || "");
                    if (!urlCtor) return { sig: null, n: null };

                    var proto = Object.getPrototypeOf(urlCtor);
                    var props = Object.getOwnPropertyNames(proto);
                    var blacklist = ['constructor', 'clone', 'set', 'get'];
                    for (var i = 0; i < props.length; i++) {
                        var p = props[i];
                        if (blacklist.indexOf(p) !== -1) continue;
                        if (typeof urlCtor[p] === 'function') {
                            try {
                                urlCtor[p]();
                            } catch (e) {}
                        }
                    }
                    var sigRes = sp ? urlCtor.get(sp) : urlCtor.get('signature');
                    var nRes = urlCtor.get('n');
                    return {
                        sig: sigRes ? decodeURIComponent(sigRes) : null,
                        n: nRes ? decodeURIComponent(nRes) : null
                    };
                }
            "#,
            )
            .map_err(|e| InnertubeError::Player(format!("Failed to define __process_nsig: {e}")))?;

            Ok(())
        })?;

        Ok(Self {
            _runtime: runtime,
            context,
            nsig_fn_name,
            signature_timestamp: sts,
        })
    }

    /// Run decipher transform on signature (`s`), signature parameter (`sp`), and n-token (`n`).
    pub fn decipher(
        &self,
        n: Option<&str>,
        sp: Option<&str>,
        s: Option<&str>,
    ) -> Result<DecipherResult> {
        self.context.with(|ctx| {
            let global = ctx.globals();
            let nsig_fn: Function = global
                .get("__nsig_fn")
                .map_err(|e| InnertubeError::Player(format!("Failed to get __nsig_fn from window: {e}")))?;
            let process_fn: Function = global
                .get("__process_nsig")
                .map_err(|e| InnertubeError::Player(format!("Failed to get process function: {e}")))?;

            let res_obj: Object = process_fn
                .call((
                    nsig_fn,
                    n.unwrap_or(""),
                    sp.unwrap_or(""),
                    s.unwrap_or(""),
                ))
                .map_err(|e| InnertubeError::Player(format!("Decipher JS call failed: {e}")))?;

            let sig: Option<String> = res_obj.get("sig").unwrap_or(None);
            let n_res: Option<String> = res_obj.get("n").unwrap_or(None);

            Ok(DecipherResult { sig, n: n_res })
        })
    }

    /// Apply signature and n-token transformations directly to a YouTube streaming URL.
    pub fn apply_to_url(&self, raw_url: &str, sp_param: Option<&str>, sig: Option<&str>) -> Result<String> {
        self.decipher_stream_url(raw_url, sp_param, sig, None, None)
    }

    /// Full legacy `Player.decipher` URL pipeline: n-token transform (with
    /// optional per-response cache), signature application (`sp` param or
    /// `signature` when absent), `pot` PO-token append (skipped for SABR
    /// URLs), and `cver` rewrite based on the `c` client param.
    pub fn decipher_stream_url(
        &self,
        raw_url: &str,
        sp_param: Option<&str>,
        sig: Option<&str>,
        po_token: Option<&str>,
        nsig_cache: Option<&mut NsigCache>,
    ) -> Result<String> {
        let parsed_url = url::Url::parse(raw_url)
            .map_err(|e| InnertubeError::Format(format!("Invalid stream URL: {e}")))?;

        let current_n = parsed_url
            .query_pairs()
            .find(|(k, _)| k == "n")
            .map(|(_, v)| v.to_string());

        let mut n_result = current_n.as_ref().and_then(|n| {
            nsig_cache
                .as_ref()
                .and_then(|cache| cache.get(n).cloned())
        });
        let mut sig_result = None;

        let needs_eval = (current_n.is_some() && n_result.is_none()) || sig.is_some();
        if needs_eval {
            let eval_n = if n_result.is_none() {
                current_n.as_deref()
            } else {
                None
            };
            let result = self.decipher(eval_n, sp_param, sig)?;

            if let (Some(n), Some(new_n)) = (eval_n, result.n) {
                if !new_n.starts_with(NSIG_ERROR_SENTINEL) {
                    if let Some(cache) = nsig_cache {
                        cache.insert(n.to_string(), new_n.clone());
                    }
                }
                if n_result.is_none() {
                    n_result = Some(new_n);
                }
            }
            sig_result = result.sig;
        }

        finalize_stream_url(
            raw_url,
            sp_param,
            sig_result.as_deref(),
            n_result.as_deref(),
            po_token,
        )
    }
}

/// Apply decipher results and legacy URL rewrites (pot, cver) to a stream
/// URL. Pure function; performs no JS evaluation.
pub fn finalize_stream_url(
    raw_url: &str,
    sp_param: Option<&str>,
    new_sig: Option<&str>,
    new_n: Option<&str>,
    po_token: Option<&str>,
) -> Result<String> {
    let mut parsed_url = url::Url::parse(raw_url)
        .map_err(|e| InnertubeError::Format(format!("Invalid stream URL: {e}")))?;

    let mut query_pairs: Vec<(String, String)> = parsed_url
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    if let Some(new_n) = new_n {
        if let Some(pair) = query_pairs.iter_mut().find(|(k, _)| k == "n") {
            pair.1 = new_n.to_string();
        } else {
            query_pairs.push(("n".to_string(), new_n.to_string()));
        }
    }

    if let Some(new_sig) = new_sig {
        // Legacy uses the `sp` param name, or `signature` when absent.
        let key = sp_param.unwrap_or("signature").to_string();
        query_pairs.retain(|(k, _)| k != &key && k != "s");
        query_pairs.push((key, new_sig.to_string()));
    }

    // SABR requests carry the PO token in the payload instead of the URL.
    let is_sabr = query_pairs.iter().any(|(k, v)| k == "sabr" && v == "1");
    if !is_sabr {
        if let Some(pot) = po_token {
            query_pairs.retain(|(k, _)| k != "pot");
            query_pairs.push(("pot".to_string(), pot.to_string()));
        }
    }

    if let Some((_, c)) = query_pairs.iter().find(|(k, _)| k == "c").cloned() {
        if let Some(cver) = legacy_cver_for_client(&c) {
            if let Some(pair) = query_pairs.iter_mut().find(|(k, _)| k == "cver") {
                pair.1 = cver.to_string();
            } else {
                query_pairs.push(("cver".to_string(), cver.to_string()));
            }
        }
    }

    parsed_url
        .query_pairs_mut()
        .clear()
        .extend_pairs(query_pairs.iter().map(|(k, v)| (&k[..], &v[..])));

    Ok(parsed_url.to_string())
}

/// Legacy `cver` mapping for the `c` query param of stream URLs.
pub fn legacy_cver_for_client(client: &str) -> Option<&'static str> {
    match client {
        "WEB" => Some(clients::WEB_VERSION),
        "MWEB" => Some(clients::MWEB_VERSION),
        "WEB_REMIX" => Some(clients::WEB_REMIX_VERSION),
        "WEB_KIDS" => Some(clients::WEB_KIDS_VERSION),
        "TVHTML5" => Some(clients::TV_VERSION),
        "TVHTML5_SIMPLY" => Some(clients::TV_SIMPLY_VERSION),
        "TVHTML5_SIMPLY_EMBEDDED_PLAYER" => Some(clients::TV_EMBEDDED_VERSION),
        "WEB_EMBEDDED_PLAYER" => Some(clients::WEB_EMBEDDED_VERSION),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finalize_appends_pot_unless_sabr() {
        let url = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB&n=abc",
            None,
            None,
            None,
            Some("po-token-123"),
        )
        .unwrap();
        assert!(url.contains("pot=po-token-123"), "{url}");

        let sabr = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB&sabr=1",
            None,
            None,
            None,
            Some("po-token-123"),
        )
        .unwrap();
        assert!(!sabr.contains("pot="), "{sabr}");
    }

    #[test]
    fn finalize_rewrites_cver_per_client_param() {
        let web = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB&cver=0.0",
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(web.contains(&format!("cver={}", clients::WEB_VERSION)), "{web}");

        let remix = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB_REMIX",
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(
            remix.contains(&format!("cver={}", clients::WEB_REMIX_VERSION)),
            "{remix}"
        );

        let android = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=ANDROID&cver=21.03.36",
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(android.contains("cver=21.03.36"), "{android}");
    }

    #[test]
    fn finalize_applies_signature_to_sp_or_signature_param() {
        let with_sp = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB&s=encrypted",
            Some("lsig"),
            Some("decoded-sig"),
            None,
            None,
        )
        .unwrap();
        assert!(with_sp.contains("lsig=decoded-sig"), "{with_sp}");
        assert!(!with_sp.contains("s=encrypted"), "{with_sp}");

        let without_sp = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB&s=encrypted",
            None,
            Some("decoded-sig"),
            None,
            None,
        )
        .unwrap();
        assert!(without_sp.contains("signature=decoded-sig"), "{without_sp}");
        assert!(!without_sp.contains("s=encrypted"), "{without_sp}");
    }

    #[test]
    fn finalize_replaces_n_token() {
        let url = finalize_stream_url(
            "https://rr1.googlevideo.com/videoplayback?c=WEB&n=old-token",
            None,
            None,
            Some("new-token"),
            None,
        )
        .unwrap();
        assert!(url.contains("n=new-token"), "{url}");
        assert!(!url.contains("n=old-token"), "{url}");
    }

    #[test]
    fn nsig_error_sentinel_results_are_not_cacheable() {
        assert!("enhanced_except_xyz".starts_with(NSIG_ERROR_SENTINEL));
        assert!(!"valid-token".starts_with(NSIG_ERROR_SENTINEL));
    }

    #[test]
    fn missing_signature_timestamp_maps_to_none() {
        // Extraction failure must not silently default to 0 (legacy warns and
        // continues without the field).
        let re = regex::Regex::new(r"signatureTimestamp\s*:\s*(\d+)").unwrap();
        assert!(re.captures("var a=1;").is_none());
        assert!(PlayerDecipherer::new("var a=1;").is_err());
    }
}
