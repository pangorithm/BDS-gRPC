fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/hello.proto",
                "BDS-proto/common/key_value.proto",
                "BDS-proto/packet.proto",
            ],
            &["proto", "BDS-proto"], // proto 파일이 있는 디렉토리 지정
        )?;
    Ok(())
}
