use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rquickjs::{CatchResultExt, Context, Runtime};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Full BotGuard + Google GenerateIT Flow ===");

    let client = reqwest::Client::new();
    let url = "https://jnn-pa.googleapis.com/$rpc/google.internal.waa.v1.Waa/Create";
    let payload = json!(["O43z0dpjhgX20SCx4KAo"]);

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
    let request_key = challenge_json.get(0).and_then(|v| v.as_str()).unwrap_or_default();
    let program = challenge_json.get(4).and_then(|v| v.as_str()).unwrap_or_default();
    let global_name = challenge_json.get(5).and_then(|v| v.as_str()).unwrap_or_default();
    let script = challenge_json.get(1).and_then(|v| v.as_array()).and_then(|arr| arr.iter().find_map(|v| v.as_str())).unwrap_or_default();

    println!("Request Key: {}", request_key);
    println!("Global Name: {}", global_name);

    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    let botguard_token: String = ctx.with(|ctx| {
        let env_setup = r#"
            var window = globalThis;
            var self = globalThis;
            var top = globalThis;
            var parent = globalThis;
            var document = {
                createElement: function(tag) { return { setAttribute: function(){}, getAttribute: function(){ return null; }, style: {} }; },
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
                    return snapResult || "NO_TOKEN";
                }} catch(err) {{
                    return "ERROR: " + err;
                }}
            }})()
        "#, gn = global_name, prog = program);

        ctx.eval::<String, _>(eval_minter).catch(&ctx).unwrap()
    });

    println!("BotGuard Snapshot Token: {}", &botguard_token[..std::cmp::min(40, botguard_token.len())]);

    // Request GenerateIT
    let it_url = "https://jnn-pa.googleapis.com/$rpc/google.internal.waa.v1.Waa/GenerateIT";
    let it_resp = client.post(it_url)
        .header("content-type", "application/json+protobuf")
        .header("x-goog-api-key", "AIzaSyDyT5W0Jh49F30Pqqtyfdf7pDLFKLJoAnw")
        .header("x-user-agent", "grpc-web-javascript/0.1")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .json(&json!([request_key, botguard_token]))
        .send().await?;

    let it_json: Value = it_resp.json().await?;
    println!("Google Integrity Token Response:\n{}", serde_json::to_string_pretty(&it_json)?);

    Ok(())
}
