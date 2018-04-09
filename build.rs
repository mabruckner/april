extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    let mut build = cc::Build::new();
    build.include("apriltag-2016-12-01");
    println!("DO STUFF");
    for file in fs::read_dir("apriltag-2016-12-01").unwrap().chain(fs::read_dir("apriltag-2016-12-01/common").unwrap()) {
        if let Ok(file) = file {
            println!("FILE {:?}", file.path());
            if file.path().extension().map(|x| x.to_str().unwrap()) == Some("c") {
                println!("BUILDING {:?}", file.path());
                build.file(file.path());
            }
        }
    }
    build.compile("libapril.a");
    println!("DONE STUFF");

    //println!("cargo:rustc-link-lib=static=apriltag");
    //println!("cargo:rustc-link-search={}/apriltag-2016-12-01", env::var("CARGO_MANIFEST_DIR").unwrap() );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
