pub mod day1;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let result = day1::run2(input);

    println!("{result}");
}
