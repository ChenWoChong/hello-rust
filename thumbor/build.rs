use std::fs;

const PROTO_PATH: &'static str = "src/pb";

fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);
    if !build_enabled {
        println!("=== Skipped compiling proto ===");
        return;
    }

    init_dir(PROTO_PATH);
    prost_build::Config::new()
        .out_dir(PROTO_PATH)
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}

fn init_dir(path: &str) {
    match fs::create_dir_all(path) {
        Ok(_) => println!("folder existed {}", path),
        Err(e) => println!("create folder err {}", e),
    }
}
