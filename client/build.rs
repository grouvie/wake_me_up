use std::{io::Result, path::PathBuf};

fn main() -> Result<()> {
    //std::env::set_var("PROTOC", protobuf_src::protoc());

    //println!("{:#?}", protobuf_src::protoc());

    let proto_files = [
        "protos/basic_response.proto",
        "protos/device.proto",
        "protos/login_request.proto",
        "protos/login_response.proto",
        "protos/wake_up.proto",
    ];

    // Convert relative paths to absolute paths
    let mut absolute_proto_files: Vec<PathBuf> = Vec::new();
    for proto_file in proto_files.iter() {
        let mut proto_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        proto_path.pop(); // Remove the 'client' directory
                          // proto_path.push("protos");
        proto_path.push(proto_file);
        absolute_proto_files.push(proto_path);
    }

    println!("{:#?}", absolute_proto_files);

    let mut includes_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    includes_path.pop(); // Remove the 'client' directory
    includes_path.push("protos/");

    let includes_path_str = includes_path
        .to_str()
        .expect("Failed to convert includes path to string");

    println!("{}", includes_path_str);

    let includes = &[includes_path_str];

    prost_build::compile_protos(
        &absolute_proto_files
            .iter()
            .map(|p| p.to_str().expect("Failed to convert path to string"))
            .collect::<Vec<&str>>(),
        includes,
    )?;

    Ok(())
}
