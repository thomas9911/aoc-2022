mod bindings;

use std::ffi::{CStr, CString};
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

/// copied from nightly rust
fn from_bytes_until_nul(bytes: &[u8]) -> Result<&CStr, ()> {
    let nul_pos = bytes.iter().position(|x| x == &0);
    match nul_pos {
        Some(nul_pos) => {
            let subslice = &bytes[..nul_pos + 1];
            // SAFETY: We know there is a nul byte at nul_pos, so this slice
            // (ending at the nul byte) is a well-formed C string.
            Ok(unsafe { CStr::from_bytes_with_nul_unchecked(subslice) })
        }
        None => Err(()),
    }
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

// pub fn day03a() -> i32 {
//     let s = read_to_cstring("data/day03/day03a.txt").unwrap();
//     unsafe { bindings::day03a(s.as_ptr()) }
// }

// pub fn day03b() -> i32 {
//     let s = read_to_cstring("data/day03/day03b.txt").unwrap();
//     unsafe { bindings::day03b(s.as_ptr()) }
// }

pub fn day04a() -> i32 {
    let s = read_to_cstring("data/day04/day04a.txt").unwrap();
    unsafe { bindings::day04a(s.as_ptr()) }
}

pub fn day04b() -> i32 {
    let s = read_to_cstring("data/day04/day04b.txt").unwrap();
    unsafe { bindings::day04b(s.as_ptr()) }
}

pub fn day06a() -> i32 {
    let s = read_to_cstring("data/day06/day06a.txt").unwrap();
    unsafe { bindings::day06a(s.as_ptr()) }
}

pub fn day06b() -> i32 {
    let s = read_to_cstring("data/day06/day06b.txt").unwrap();
    unsafe { bindings::day06b(s.as_ptr()) }
}

// pub fn day07a() -> i32 {
//     let s = read_to_cstring("data/day07/day07a.txt").unwrap();
//     unsafe { bindings::day07a(s.as_ptr()) }
// }

// pub fn day07b() -> i32 {
//     let s = read_to_cstring("data/day07/day07b.txt").unwrap();
//     unsafe { bindings::day07b(s.as_ptr()) }
// }

pub fn day08a() -> i32 {
    let s = read_to_cstring("data/day08/day08a.txt").unwrap();
    unsafe { bindings::day08a(s.as_ptr()) }
}

pub fn day08b() -> i32 {
    let s = read_to_cstring("data/day08/day08b.txt").unwrap();
    unsafe { bindings::day08b(s.as_ptr()) }
}

pub fn day09a() -> i32 {
    let s = read_to_cstring("data/day09/day09a.txt").unwrap();
    unsafe { bindings::day09a(s.as_ptr()) }
}

pub fn day09b() -> i32 {
    let s = read_to_cstring("data/day09/day09b.txt").unwrap();
    unsafe { bindings::day09b(s.as_ptr()) }
}

pub fn day10a() -> i32 {
    let s = read_to_cstring("data/day10/day10a.txt").unwrap();
    unsafe { bindings::day10a(s.as_ptr()) }
}

pub fn day10b() -> String {
    let mut string_buf: Vec<u8> = vec![0; 256];
    let s = read_to_cstring("data/day10/day10b.txt").unwrap();
    unsafe {
        bindings::day10b(s.as_ptr(), string_buf.as_mut_ptr());

        from_bytes_until_nul(string_buf.as_ref())
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}
