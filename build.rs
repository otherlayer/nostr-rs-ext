fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &["../nostr-rs-relay/proto/nauthz.proto"],
            &["../nostr-rs-relay/proto"],
        )?;
    Ok(())
}
