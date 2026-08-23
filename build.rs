fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path().map_err(|e| format!("Failed to get vendored protoc: {e}"))?;
    std::env::set_var("PROTOC", protoc_path);

    let mut config = prost_build::Config::new();
    config.compile_protos(
        &["protos/misc/params.proto", "protos/misc/common.proto"],
        &["protos/misc", "protos"],
    )?;

    println!("cargo:rerun-if-changed=protos/misc/params.proto");
    println!("cargo:rerun-if-changed=protos/misc/common.proto");

    Ok(())
}
