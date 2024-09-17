    // Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/main.rs");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
    use std::collections::HashMap; // HashMap has been imported.
let map: HashMap<u32, u32> = HashMap::new(); // So it can be used!
        .file("src/main.rs")
        .compile("main.rs");
}