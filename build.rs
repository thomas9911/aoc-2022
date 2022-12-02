use std::path::PathBuf;

static SKIP_FILES: [&str; 1] = ["test.c"];

/// selects the root folder of the include repos
fn includes() -> impl std::iter::Iterator<Item = PathBuf> {
    std::fs::read_dir("include")
        .unwrap()
        .flat_map(|folder_item| {
            let entry = folder_item.unwrap();
            if entry.file_type().unwrap().is_dir() {
                Some(entry.path())
            } else {
                None
            }
        })
}

/// selects the c files in the root folder of the include repos
fn compile_files() -> impl std::iter::Iterator<Item = PathBuf> {
    std::fs::read_dir("include").unwrap().flat_map(
        |folder_item| -> Box<dyn std::iter::Iterator<Item = PathBuf>> {
            let entry = folder_item.unwrap();
            if entry.file_type().unwrap().is_dir() {
                Box::new(
                    entry
                        .path()
                        .read_dir()
                        .unwrap()
                        .flat_map(|nested_folder_item| {
                            let entry = nested_folder_item.unwrap();
                            if entry.file_type().unwrap().is_file() {
                                let file_name_os_str = entry.file_name();
                                let file_name = file_name_os_str.to_string_lossy();
                                if file_name.ends_with(".c") && !SKIP_FILES.contains(&&*file_name) {
                                    Some(entry.path())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }),
                )
            } else {
                Box::new(std::iter::empty::<PathBuf>())
            }
        },
    )
}

fn main() {
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rerun-if-changed=lib");
    println!("cargo:rerun-if-changed=include");

    cc::Build::new()
        .warnings(false)
        .include("lib")
        .includes(includes())
        .files(compile_files())
        .file("lib/wrapper.c")
        .compile("wrapper");
}
