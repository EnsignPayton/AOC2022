pub mod day1;
pub mod day2;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let result = day2::run2(input);

    println!("{result}");
}
