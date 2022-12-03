pub mod wrapper;

#[cfg(test)]
use wrapper::*;

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

#[test]
fn day02a_test() {
    assert_eq!(12679, day02a());
}

#[test]
fn day02b_test() {
    assert_eq!(14470, day02b());
}

#[test]
fn day03a_test() {
    assert_eq!(1234, day03a());
}

#[test]
fn day03b_test() {
    assert_eq!(1234, day03b());
}
