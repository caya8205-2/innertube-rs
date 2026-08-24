use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rquickjs::{CatchResultExt, Context, Runtime};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Native BotGuard Challenge Execution in QuickJS ===");

    let client = reqwest::Client::new();
    let url = "https://jnn-pa.googleapis.com/$rpc/google.internal.waa.v1.Waa/Create";
    let payload = serde_json::json!(["O43z0dpjhgX20SCx4KAo"]);

    let resp = client.post(url)
        .header("content-type", "application/json+protobuf")
        .header("x-goog-api-key", "AIzaSyDyT5W0Jh49F30Pqqtyfdf7pDLFKLJoAnw")
        .header("x-user-agent", "grpc-web-javascript/0.1")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .json(&payload)
        .send().await?;

    let raw_data: Value = resp.json().await?;
    let scrambled = raw_data.get(1).and_then(|v| v.as_str()).unwrap_or_default();
    let decoded_bytes = BASE64.decode(scrambled)?;
    let descrambled: String = decoded_bytes.into_iter().map(|b| (b.wrapping_add(97)) as char).collect();

    let challenge_json: Value = serde_json::from_str(&descrambled)?;
    let program = challenge_json.get(4).and_then(|v| v.as_str()).unwrap_or_default();
    let global_name = challenge_json.get(5).and_then(|v| v.as_str()).unwrap_or_default();
    let script = challenge_json.get(1).and_then(|v| v.as_array()).and_then(|arr| arr.iter().find_map(|v| v.as_str())).unwrap_or_default();

    println!("Global Name: {}", global_name);
    println!("Program length: {}", program.len());

    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    let snapshot_res: String = ctx.with(|ctx| {
        let env_setup = r#"
            var window = globalThis;
            var self = globalThis;
            var top = globalThis;
            var parent = globalThis;
            var document = {
                createElement: function() { return { setAttribute: function(){}, getAttribute: function(){ return null; }, style: {} }; },
                getElementsByTagName: function() { return []; },
                querySelectorAll: function() { return []; },
                querySelector: function() { return null; },
                documentElement: { style: {} },
                body: { style: {} },
                cookie: ""
            };
            var navigator = {
                userAgent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                plugins: [],
                languages: ["en-US", "en"],
                webdriver: false,
                hardwareConcurrency: 8,
                deviceMemory: 8
            };
            var location = {
                href: "https://www.youtube.com",
                hostname: "www.youtube.com",
                protocol: "https:",
                origin: "https://www.youtube.com"
            };
            var screen = { width: 1920, height: 1080, colorDepth: 24 };
            var performance = { now: function() { return Date.now(); } };

            function TextEncoder() {}
            TextEncoder.prototype.encode = function(str) {
                var bytes = [];
                for (var i = 0; i < str.length; i++) {
                    bytes.push(str.charCodeAt(i));
                }
                return new Uint8Array(bytes);
            };

            function btoa(str) {
                var chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=';
                var encoded = '';
                var c1, c2, c3;
                var e1, e2, e3, e4;
                var i = 0;
                while (i < str.length) {
                    c1 = str.charCodeAt(i++);
                    c2 = str.charCodeAt(i++);
                    c3 = str.charCodeAt(i++);
                    e1 = c1 >> 2;
                    e2 = ((c1 & 3) << 4) | (c2 >> 4);
                    e3 = isNaN(c2) ? 64 : (((c2 & 15) << 2) | (c3 >> 6));
                    e4 = isNaN(c3) ? 64 : (c3 & 63);
                    encoded += chars.charAt(e1) + chars.charAt(e2) + chars.charAt(e3) + chars.charAt(e4);
                }
                return encoded;
            }
        "#;
        ctx.eval::<(), _>(env_setup).catch(&ctx).unwrap();
        ctx.eval::<(), _>(script).catch(&ctx).unwrap();

        let eval_minter = format!(r#"
            (function() {{
                try {{
                    var vm = globalThis["{gn}"];
                    if (!vm || !vm.a) return "VM_NO_A";
                    var asyncSnapshot = null;
                    var setupCb = function(asyncFn) {{
                        asyncSnapshot = asyncFn;
                    }};
                    var loggers = [function(){{}}, function(){{}}, function(){{}}, function(){{}}, function(){{}}];
                    vm.a("{prog}", setupCb, true, null, function(){{}}, [[], []], undefined, false, loggers);

                    var snapResult = null;
                    if (asyncSnapshot) {{
                        asyncSnapshot(function(response) {{
                            snapResult = response;
                        }}, ["e1bCibq2I1g", null, [], false]);
                    }}
                    if (!snapResult) return "NO_SNAP_RESULT";
                    return snapResult;
                }} catch(err) {{
                    return "ERROR: " + err;
                }}
            }})()
        "#, gn = global_name, prog = program);

        ctx.eval::<String, _>(eval_minter).catch(&ctx).unwrap()
    });

    println!("BotGuard Snapshot Token Result:\n{}", snapshot_res);
    Ok(())
}
