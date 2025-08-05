// Build script for Cap'n Proto schema compilation
// We'll add Cap'n Proto compilation here later

fn main() {
    println!("cargo:rerun-if-changed=schema/");
}