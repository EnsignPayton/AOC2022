pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let result = day5::run2(input.as_str());

    println!("{result}");
}
