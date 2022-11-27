use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rerun-if-changed=lib");

    let bindings = bindgen::Builder::default()
        .header("lib/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .include("lib")
        .include("include")
        .file("include/clist.c")
        .file("lib/wrapper.c")
        .compile("wrapper");
}
