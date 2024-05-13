
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("building...");
    let proto_file = "./proto/demo.proto";

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems below 3.21
        .build_client(true)
        .build_server(true)
        .out_dir("./src")
        .compile(&[proto_file], &["proto"])?;

    Ok(())
}
