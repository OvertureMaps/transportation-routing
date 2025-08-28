use std::path::PathBuf;
use std::env;

fn main() {
    let bindings_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    println!("cargo:rerun-if-changed=c_code/valhalla.h");
    bindgen::Builder::default()
        .header("c_code/valhalla.h")
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");
}
