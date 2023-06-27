pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let result = day4::run2(input);

    println!("{result}");
}
