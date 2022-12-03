import sys
import os
import subprocess

HELP = """

Placeholder text

"""


def pad_day(day):
    return str(day).rjust(2, "0")


def new_file(path):
    open(path, "a").close()


def bootstrap_data(day):
    os.mkdir(f"data/day{day}")
    new_file(f"data/day{day}/day{day}a.txt")
    new_file(f"data/day{day}/day{day}b.txt")


def bootstrap_lib(day):
    os.mkdir(f"lib/day{day}")
    with open(f"lib/day{day}/lib.c", "w") as f:
        f.write(
            f"""
long day{day}a(const char *text) {{
    return 123;
}}

long day{day}b(const char *text) {{
    return 123;
}}
"""
        )

    with open(f"lib/wrapper.c", "a") as f:
        f.write(f"#include <day{day}/lib.c>\n")

    with open(f"lib/wrapper.h", "a") as f:
        f.write(f"\nlong day{day}a(const char *text);\n")
        f.write(f"long day{day}b(const char *text);\n")


def bootstrap_src(day):
    with open(f"src/wrapper.rs", "a") as f:
        f.write(
            f"""
pub fn day{day}a() -> i32 {{
    let s = read_to_cstring("data/day{day}/day{day}a.txt").unwrap();
    unsafe {{ bindings::day{day}a(s.as_ptr()) }}
}}

pub fn day{day}b() -> i32 {{
    let s = read_to_cstring("data/day{day}/day{day}b.txt").unwrap();
    unsafe {{ bindings::day{day}b(s.as_ptr()) }}
}}
"""
        )

    with open(f"src/lib.rs", "a") as f:
        f.write(
            f"""
#[test]
fn day{day}a_test() {{
    assert_eq!(1234, day{day}a());
}}

#[test]
fn day{day}b_test() {{
    assert_eq!(1234, day{day}b());
}}
"""
        )


def generate_bindings():
    my_env = os.environ.copy()
    clang_path = r"D:\\Program Files\LLVM\bin\libclang.dll"
    if os.path.exists(clang_path):
        # fix for my weird windows install
        my_env["LIBCLANG_PATH"] = r"D:\\Program Files\LLVM\bin\libclang.dll"

    subprocess.run(["cargo", "install", "bindgen-cli"])
    subprocess.run(
        [
            "bindgen",
            "-o",
            "src/wrapper/bindings.rs",
            "--generate",
            "functions,methods,constructors,destructors",
            "lib/wrapper.h",
        ],
        env=my_env,
    )


def main():
    args = sys.argv
    if len(args) != 2:
        return print(HELP)

    if args[1] in ["-h", "--help", "help"]:
        return print(HELP)

    try:
        day = int(args[1])
    except ValueError:
        return print(HELP)

    if day > 31 or day < 0:
        return print(HELP)

    day = pad_day(day)
    bootstrap_data(day)
    bootstrap_lib(day)
    bootstrap_src(day)
    generate_bindings()


if __name__ == "__main__":
    main()
