fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path().map_err(|e| format!("Failed to get vendored protoc: {e}"))?;
    std::env::set_var("PROTOC", protoc_path);

    let pfiinnertube_protos: Vec<String> = std::fs::read_dir("protos/youtube/api/pfiinnertube")?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension().and_then(|e| e.to_str()) == Some("proto"))
                .then(|| path.to_string_lossy().replace('\\', "/"))
        })
        .collect();

    let mut protos = vec!["misc/params.proto".to_string(), "misc/common.proto".to_string()];
    protos.extend(pfiinnertube_protos);

    let mut config = prost_build::Config::new();
    config.compile_protos(&protos, &["protos"])?;

    println!("cargo:rerun-if-changed=protos/misc/params.proto");
    println!("cargo:rerun-if-changed=protos/misc/common.proto");
    println!("cargo:rerun-if-changed=protos/youtube/api/pfiinnertube");

    Ok(())
}
