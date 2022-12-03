mod bindings;

use std::ffi::CString;
use std::fs::read_to_string;

fn read_to_cstring(path: &str) -> Result<CString, Box<dyn std::error::Error>> {
    let mut input = read_to_string(path)?
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .join("\n");
    input.push('\n');
    let s = CString::new(input)?;
    Ok(s)
}

pub fn day0a() -> i32 {
    let s = read_to_cstring("data/day0/day0a.txt").unwrap();
    unsafe { bindings::day0a(s.as_ptr()) }
}

pub fn day0b() -> i32 {
    let s = read_to_cstring("data/day0/day0b.txt").unwrap();
    unsafe { bindings::day0b(s.as_ptr()) }
}

pub fn day0c() -> i32 {
    let s = read_to_cstring("data/day0/day0c.txt").unwrap();
    unsafe { bindings::day0c(s.as_ptr()) }
}

pub fn day0d() -> i32 {
    let s = read_to_cstring("data/day0/day0d.txt").unwrap();
    unsafe { bindings::day0d(s.as_ptr()) }
}

pub fn day01a() -> i32 {
    let s = read_to_cstring("data/day01/day01a.txt").unwrap();
    unsafe { bindings::day01a(s.as_ptr()) }
}

pub fn day01b() -> i32 {
    let s = read_to_cstring("data/day01/day01b.txt").unwrap();
    unsafe { bindings::day01b(s.as_ptr()) }
}

pub fn day02a() -> i32 {
    let s = read_to_cstring("data/day02/day02a.txt").unwrap();
    unsafe { bindings::day02a(s.as_ptr()) }
}

pub fn day02b() -> i32 {
    let s = read_to_cstring("data/day02/day02b.txt").unwrap();
    unsafe { bindings::day02b(s.as_ptr()) }
}

pub fn day03a() -> i32 {
    let s = read_to_cstring("data/day03/day03a.txt").unwrap();
    unsafe { bindings::day03a(s.as_ptr()) }
}

pub fn day03b() -> i32 {
    let s = read_to_cstring("data/day03/day03b.txt").unwrap();
    unsafe { bindings::day03b(s.as_ptr()) }
}
