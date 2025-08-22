fn main() {
    println!("cargo:rerun-if-changed=src/valhalla.h");
    let bindings = bindgen::Builder::default()
        .header("src/valhalla.h")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
