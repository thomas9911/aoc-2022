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

pub fn bar_function(x: i32) -> i32 {
    unsafe { bindings::bar_function(x) }
}

pub fn max(x: i32, y: i32) -> i32 {
    unsafe { bindings::max_impl(x, y) }
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
    assert_eq!(1601, day0b());
}

#[test]
fn day0c_test() {
    assert_eq!(1868935, day0c());
}

#[test]
fn day0d_test() {
    assert_eq!(1965970888, day0d());
}

#[test]
fn day01a_test() {
    assert_eq!(69795, day01a());
}

#[test]
fn day01b_test() {
    assert_eq!(208437, day01b());
}
