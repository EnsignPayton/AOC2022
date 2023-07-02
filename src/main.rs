pub mod day5;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let result = day5::run2(input.as_str());

    println!("{result}");
}
