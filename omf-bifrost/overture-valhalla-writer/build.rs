use std::fs;

fn main() {
    // Tell Cargo to look for changes in the `c` directory
    println!("cargo:rerun-if-changed=c");

    // Create a new `cc::Build`
    let mut build = cc::Build::new();

    // Iterate over all `.c` files in the `c` directory
    for entry in fs::read_dir("c").unwrap() {
        let path = entry.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "c" {
                // Add the `.c` file to the build process
                build.file(path);
            }
        }
    }

    // Compile the C code
    build.compile("c_lib"); // Name of the output library
}

