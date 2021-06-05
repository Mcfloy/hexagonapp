fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(
            &[
                "src/application/grpc/proto/student.proto"
            ],
            &[
                "src/application/grpc/proto"
            ]
        )?;
    Ok(())
}