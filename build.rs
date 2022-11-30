fn main() {
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rerun-if-changed=lib");
    println!("cargo:rerun-if-changed=include");

    let includes = std::fs::read_dir("include").unwrap().flat_map(|x| {
        let entry = x.unwrap();
        if entry.file_type().unwrap().is_dir() {
            Some(entry.path())
        } else {
            None
        }
    });

    cc::Build::new()
        .include("lib")
        .includes(includes)
        .file("include/CLIST/clist.c")
        .file("include/hashmap.c/hashmap.c")
        .file("lib/wrapper.c")
        .compile("wrapper");
}
