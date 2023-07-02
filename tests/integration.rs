use aoc_2022::*;

#[test]
fn day1_run1() {
    let input = include_str!("../test_input/day1.txt");
    let result = day1::run1(input);
    assert_eq!(result, 24000);
}

#[test]
fn day1_run2() {
    let input = include_str!("../test_input/day1.txt");
    let result = day1::run2(input);
    assert_eq!(result, 45000);
}

#[test]
fn day2_run1() {
    let input = include_str!("../test_input/day2.txt");
    let result = day2::run1(input);
    assert_eq!(result, 15);
}

#[test]
fn day2_run2() {
    let input = include_str!("../test_input/day2.txt");
    let result = day2::run2(input);
    assert_eq!(result, 12);
}

#[test]
fn day3_run1() {
    let input = include_str!("../test_input/day3.txt");
    let result = day3::run1(input);
    assert_eq!(result, 157);
}

#[test]
fn day3_run2() {
    let input = include_str!("../test_input/day3.txt");
    let result = day3::run2(input);
    assert_eq!(result, 70);
}

#[test]
fn day4_run1() {
    let input = include_str!("../test_input/day4.txt");
    let result = day4::run1(input);
    assert_eq!(result, 2);
}

#[test]
fn day4_run2() {
    let input = include_str!("../test_input/day4.txt");
    let result = day4::run2(input);
    assert_eq!(result, 4);
}

#[test]
fn day5_run1() {
    let input = include_str!("../test_input/day5.txt");
    let result = day5::run1(input);
    assert_eq!(result, "CMZ");
}

#[test]
fn day5_run2() {
    let input = include_str!("../test_input/day5.txt");
    let result = day5::run2(input);
    assert_eq!(result, "MCD");
}
