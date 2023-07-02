use aoc_2022::*;

const DAY1: &'static str = include_str!("data/day1.txt");
const DAY2: &'static str = include_str!("data/day2.txt");
const DAY3: &'static str = include_str!("data/day3.txt");
const DAY4: &'static str = include_str!("data/day4.txt");
const DAY5: &'static str = include_str!("data/day5.txt");

#[test]
fn day1_run1() {
    assert_eq!(day1::run1(DAY1), "24000");
}

#[test]
fn day1_run2() {
    assert_eq!(day1::run2(DAY1), "45000");
}

#[test]
fn day2_run1() {
    assert_eq!(day2::run1(DAY2), "15");
}

#[test]
fn day2_run2() {
    assert_eq!(day2::run2(DAY2), "12");
}

#[test]
fn day3_run1() {
    assert_eq!(day3::run1(DAY3), "157");
}

#[test]
fn day3_run2() {
    assert_eq!(day3::run2(DAY3), "70");
}

#[test]
fn day4_run1() {
    assert_eq!(day4::run1(DAY4), "2");
}

#[test]
fn day4_run2() {
    assert_eq!(day4::run2(DAY4), "4");
}

#[test]
fn day5_run1() {
    assert_eq!(day5::run1(DAY5), "CMZ");
}

#[test]
fn day5_run2() {
    assert_eq!(day5::run2(DAY5), "MCD");
}
