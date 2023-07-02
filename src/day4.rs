struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn parse(input: &str) -> Range {
        let split: Vec<&str> = input.split('-').collect();
        Range {
            start: split[0].parse().unwrap(),
            end: split[1].parse().unwrap(),
        }
    }
}

struct RangePair {
    a: Range,
    b: Range,
}

impl RangePair {
    fn parse(input: &str) -> RangePair {
        let split: Vec<&str> = input.split(',').collect();
        RangePair {
            a: Range::parse(split[0]),
            b: Range::parse(split[1]),
        }
    }

    fn fully_overlapped(&self) -> bool {
        let a_contains_b = &self.b.start <= &self.a.start && &self.b.end >= &self.a.end;
        let b_contains_a = &self.a.start <= &self.b.start && &self.a.end >= &self.b.end;
        a_contains_b || b_contains_a
    }

    fn partially_overlapped(&self) -> bool {
        let a_contains_b = &self.a.start <= &self.b.start && &self.b.start <= &self.a.end;
        let b_contains_a = &self.b.start <= &self.a.start && &self.a.start <= &self.b.end;
        a_contains_b || b_contains_a
    }
}

pub fn run1(input: &str) -> String {
    input
        .lines()
        .map(|x| RangePair::parse(x))
        .filter(|x| x.fully_overlapped())
        .count()
        .to_string()
}

pub fn run2(input: &str) -> String {
    input
        .lines()
        .map(|x| RangePair::parse(x))
        .filter(|x| x.partially_overlapped())
        .count()
        .to_string()
}
