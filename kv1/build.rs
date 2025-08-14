use std::fs;

const PROTO_PATH: &'static str = "src/proto";

fn main() {
    _ = fs::create_dir_all(PROTO_PATH);
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir(PROTO_PATH)
        .compile_protos(&["abi.proto"], &["./src/proto"])
        .unwrap();
}
