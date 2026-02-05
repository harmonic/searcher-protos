fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .bytes(".")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                "../proto/auth.proto",
                "../proto/bundle.proto",
                "../proto/packet.proto",
                "../proto/searcher.proto",
                "../proto/shared.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}
