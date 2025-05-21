fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(
            &["proto/sigma-authentication/admin.proto", "proto/sigma-authentication/table_session.proto"],
            &["proto/sigma-authentication"]
        )?;

    Ok(())
}
