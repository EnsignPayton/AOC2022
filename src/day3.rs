fn get_common_letter(input: &str) -> char {
    let (a, b) = input.split_at(input.len() / 2);

    for achar in a.chars() {
        if b.contains(achar) {
            return achar;
        }
    }

    for bchar in b.chars() {
        if a.contains(bchar) {
            return bchar;
        }
    }

    panic!();
}

fn letter_priority(input: char) -> u32 {
    if input.is_ascii_lowercase() {
        (input as u32) - ('a' as u32) + 1
    } else {
        (input as u32) - ('A' as u32) + 27
    }
}

fn chunk_lines(input: String, size: usize) -> Vec<Vec<String>> {
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut temp: Vec<String> = Vec::new();

    for line in input.lines().filter(|x| x.len() > 0) {
        temp.push(line.to_string());

        if temp.len() == size {
            chunks.push(temp.clone());
            temp.clear()
        }
    }

    chunks
}

fn get_common_letter2(input: &Vec<String>) -> char {
    for letter in input[0].chars() {
        if input.iter().skip(1).all(|x| x.contains(letter)) {
            return letter;
        }
    }

    panic!();
}

pub fn run1(input: String) -> u32 {
    input
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| get_common_letter(x))
        .map(|x| letter_priority(x))
        .sum()
}

pub fn run2(input: String) -> u32 {
    chunk_lines(input, 3)
        .iter()
        .map(|x| get_common_letter2(x))
        .map(|x| letter_priority(x))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run1() {
        let input = include_str!("../test_input/day3.txt");
        let result = super::run1(input.to_string());
        assert_eq!(result, 157);
    }

    #[test]
    fn test_run2() {
        let input = include_str!("../test_input/day3.txt");
        let result = super::run2(input.to_string());
        assert_eq!(result, 70);
    }
}
