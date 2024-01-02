use oak_grpc_utils::{generate_grpc_code, CodegenOptions, ExternPath};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_grpc_code(
        ".",
        &["proto/t_oak_user_silo.proto"],
        CodegenOptions {
            build_server: true,
            extern_paths: vec![ExternPath::new(
                ".oak.crypto.v1",
                "oak_crypto::proto::oak::crypto::v1",
            )],
            ..Default::default()
        },
    )?;

    Ok(())
}
