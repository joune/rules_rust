fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut proto_compile_config = prost_build::Config::new();

    // Adds From/Into for chrono (https://lib.rs/crates/prost-wkt-build)
    proto_compile_config.extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp");
    proto_compile_config.extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration");
    proto_compile_config.extern_path(".google.protobuf.Any", "::prost_wkt_types::Any");

    tonic_build::configure()
        .compile_with_config(proto_compile_config, &["proto/helloworld.proto"], &["proto"])
        .expect("Failed to compile proto specification");
    Ok(())
}
