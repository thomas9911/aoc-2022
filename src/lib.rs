pub mod day03;
pub mod wrapper;

use std::fs::read_to_string;

#[cfg(test)]
use wrapper::*;

pub fn load_to_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut input = read_to_string(path)?
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .join("\n");
    input.push('\n');

    Ok(input)
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

#[test]
fn day02a_test() {
    assert_eq!(12679, day02a());
}

#[test]
fn day02b_test() {
    assert_eq!(14470, day02b());
}

#[test]
fn day03a_rust_test() {
    assert_eq!(7831, day03::day03a());
}

#[test]
fn day03b_rust_test() {
    assert_eq!(2683, day03::day03b());
}

// #[test]
// fn day03a_test() {
//     assert_eq!(7831, day03a());
// }

// #[test]
// fn day03b_test() {
//     assert_eq!(2683, day03b());
// }
