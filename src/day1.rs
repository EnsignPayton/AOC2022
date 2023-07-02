struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new(),
        }
    }

    fn sum_calories(&self) -> u32 {
        (&self.calories).iter().sum()
    }

    fn parse(input: &str) -> Vec<Elf> {
        let mut result = Vec::new();

        let mut pending_elf = Elf::new();
        for line in input.lines() {
            if line.is_empty() {
                result.push(pending_elf);
                pending_elf = Elf::new();
            } else {
                pending_elf.calories.push(line.parse::<u32>().unwrap())
            }
        }

        if pending_elf.calories.len() > 0 {
            result.push(pending_elf);
        }

        result
    }
}

pub fn run1(input: &str) -> u32 {
    Elf::parse(input)
        .iter()
        .map(|x| x.sum_calories())
        .max()
        .unwrap()
}

pub fn run2(input: &str) -> u32 {
    let mut sums: Vec<u32> = Elf::parse(input).iter().map(|x| x.sum_calories()).collect();
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run1() {
        let input = include_str!("../test_input/day1.txt");
        let result = super::run1(input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_run2() {
        let input = include_str!("../test_input/day1.txt");
        let result = super::run2(input);
        assert_eq!(result, 45000);
    }
}
