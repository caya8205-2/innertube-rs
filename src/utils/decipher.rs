use regex::Regex;
use rquickjs::{Context, Function, Object, Runtime};
use crate::error::{InnertubeError, Result};

pub struct DecipherResult {
    pub sig: Option<String>,
    pub n: Option<String>,
}

/// Decipher engine executing YouTube player decipher routines in QuickJS.
pub struct PlayerDecipherer {
    _runtime: Runtime,
    context: Context,
    pub nsig_fn_name: String,
    pub signature_timestamp: u32,
}

impl PlayerDecipherer {
    pub fn new(player_js: &str) -> Result<Self> {
        // 1. Extract signatureTimestamp
        let sts_re = Regex::new(r"signatureTimestamp\s*:\s*(\d+)")
            .map_err(|e| InnertubeError::Player(e.to_string()))?;

        let sts = sts_re
            .captures(player_js)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok())
            .unwrap_or(0);

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
        let mut parsed_url = url::Url::parse(raw_url)
            .map_err(|e| InnertubeError::Format(format!("Invalid stream URL: {e}")))?;

        let current_n = parsed_url
            .query_pairs()
            .find(|(k, _)| k == "n")
            .map(|(_, v)| v.to_string());

        let result = self.decipher(current_n.as_deref(), sp_param, sig)?;

        let mut query_pairs: Vec<(String, String)> = parsed_url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        if let Some(new_n) = result.n {
            if let Some(pair) = query_pairs.iter_mut().find(|(k, _)| k == "n") {
                pair.1 = new_n;
            } else {
                query_pairs.push(("n".to_string(), new_n));
            }
        }

        if let Some(new_sig) = result.sig {
            let key = sp_param.unwrap_or("sig").to_string();
            query_pairs.retain(|(k, _)| k != &key && k != "s");
            query_pairs.push((key, new_sig));
        }

        parsed_url.query_pairs_mut().clear().extend_pairs(query_pairs.iter().map(|(k, v)| (&k[..], &v[..])));

        Ok(parsed_url.to_string())
    }
}
