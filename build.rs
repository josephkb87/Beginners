    // Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
<<<<<<< HEAD
    println!("cargo::rerun-if-changed=src/main.rs");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/main.rs")
        .compile("main.rs");
=======
    println!("cargo::rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
>>>>>>> refs/remotes/origin/master
}