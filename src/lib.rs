pub mod day03;
pub mod day05;
pub mod day07;
pub mod wrapper;

use std::fs::read_to_string;

#[cfg(test)]
use wrapper::*;

pub fn load_to_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut input = read_to_string(path)?
        .trim_end()
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

#[test]
fn day04a_test() {
    assert_eq!(433, day04a());
}

#[test]
fn day04b_test() {
    assert_eq!(852, day04b());
}

#[test]
fn day05a_rust_test() {
    assert_eq!("VRWBSFZWM", day05::day05a());
}

#[test]
fn day05b_rust_test() {
    assert_eq!("RBTWJWMCF", day05::day05b());
}

#[test]
fn day06a_test() {
    assert_eq!(1640, day06a());
}

#[test]
fn day06b_test() {
    assert_eq!(3613, day06b());
}

#[test]
fn day07a_test() {
    assert_eq!(1297159, day07::day07a());
}

#[test]
fn day07b_test() {
    assert_eq!(3866390, day07::day07b());
}
