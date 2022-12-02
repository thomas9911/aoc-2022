# AOC 2022

This year in C (as long as I can :P)

Because i don't trust C build tools I bootstrap it with Rust.

## Run

If you have clang ect setup just run:

```sh
cargo test
```

for reasons my computer cannot find the libclang file so I use `build.ps1`

## Alter

when you update the wrapper.h file you should run `gen_headers.bat` (or copy the command if you are not on windws). This generates new Rust bindings. (you can also use this inside the build.rs file but for me it always regenerate this file when I ran the test, so for now I do it like this)

## Layout

- lib

  this contains the puzzel solutions C files

- src

  this contains the Rust wrapper. This contains the tests and to transforms the puzzel input so it works cross platform (windows `\r\n` to `\n`)

- build.rs

  this file compiles the C code and exposes it to the Rust compiler

- include

  this contains the 'external' libraries that I use in C. The are git submodules that point to the implementation repos

- data

  this contains the AOC puzzel input
