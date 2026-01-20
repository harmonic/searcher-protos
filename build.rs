fn main() {
    let proto_root = "proto";
    println!("cargo:rerun-if-changed={}", proto_root);

    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(
            &[
                "proto/auth.proto",
                "proto/searcher.proto",
            ],
            &[proto_root],
        )
        .expect("Failed to compile protos");
}
