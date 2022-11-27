use std::ffi::CString;
use std::fs::read_to_string;

fn read_to_cstring(path: &str) -> Result<CString, Box<dyn std::error::Error>> {
    let input = read_to_string(path)?
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .join("\n");
    let s = CString::new(input)?;
    Ok(s)
}

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn bar_function(x: i32) -> i32 {
    unsafe { bindings::bar_function(x) }
}

pub fn max(x: i32, y: i32) -> i32 {
    unsafe { bindings::max(x, y) }
}

pub fn day0a() -> i32 {
    let s = read_to_cstring("data/day0/day0a.txt").unwrap();

    unsafe { bindings::day0a(s.as_ptr()) }
}

pub fn day0b() -> i32 {
    let s = read_to_cstring("data/day0/day0b.txt").unwrap();

    unsafe { bindings::day0b(s.as_ptr()) }
}

#[test]
fn max_test() {
    assert_eq!(2, max(2, 1));
    assert_eq!(5, max(2, 5));
}

#[test]
fn bar_function_test() {
    assert_eq!(2, bar_function(2));
}

#[test]
fn day0a_test() {
    assert_eq!(1559, day0a());
}

#[test]
fn day0b_test() {
    assert_eq!(1600, day0b());
}
