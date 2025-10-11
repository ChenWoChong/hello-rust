fn main() {
    println!("cargo:rustc-link-lib=bz2");

    println!("cargo:rerun-if-changed=wrapper.sh");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Failed to write bindings");
}
