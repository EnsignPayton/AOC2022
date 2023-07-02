struct Move {
    count: u32,
    source: u32,
    dest: u32,
}

impl Move {
    fn parse(value: &str) -> Move {
        let parts: Vec<u32> = value
            .split(' ')
            .map(|x| x.parse::<u32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        Move {
            count: parts[0],
            source: parts[1],
            dest: parts[2],
        }
    }
}

struct Cargo {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Cargo {
    fn parse_stacks(value: &str) -> Vec<Vec<char>> {
        let stack_lines: Vec<&str> = value.lines().take_while(|x| x.len() > 0).collect();
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stack_count = (stack_lines.last().unwrap().len() as f32 / 4.0).ceil() as u32;
        for _ in 0..stack_count {
            stacks.push(Vec::new());
        }

        for line in stack_lines {
            // Indices of all the brackets
            // 0 -> 0, 4 -> 1, 8 -> 2, etc
            let ix: Vec<_> = line.match_indices("[").map(|x| x.0).collect();
            for i in ix {
                let stack_index = i / 4;
                let c = line.chars().nth(i + 1).unwrap();
                stacks[stack_index].push(c);
            }
        }

        // Now, reverse so stacks start from the bottom
        for stack in &mut stacks {
            stack.reverse()
        }

        stacks
    }

    fn parse_moves(value: &str) -> Vec<Move> {
        value
            .lines()
            .skip_while(|x| x.len() > 0)
            .skip(1)
            .map(|x| Move::parse(x))
            .collect()
    }

    fn parse(value: &str) -> Cargo {
        Cargo {
            stacks: Cargo::parse_stacks(value),
            moves: Cargo::parse_moves(value),
        }
    }

    fn move_crate(stacks: &mut Vec<Vec<char>>, source: u32, dest: u32) {
        let val = &stacks[(source - 1) as usize].pop().unwrap();
        stacks[(dest - 1) as usize].push(*val);
    }

    fn perform_move(stacks: &mut Vec<Vec<char>>, mv: &Move) {
        for _ in 0..mv.count {
            Cargo::move_crate(stacks, mv.source, mv.dest);
        }
    }

    fn perform_moves(&mut self) {
        for mv in &self.moves {
            Cargo::perform_move(&mut self.stacks, mv);
        }
    }

    fn perform_move_2(stacks: &mut Vec<Vec<char>>, mv: &Move) {
        let mut temp: Vec<char> = Vec::new();

        for _ in 0..mv.count {
            let val = &stacks[(mv.source - 1) as usize].pop().unwrap();
            temp.push(*val)
        }

        temp.reverse();

        for val in temp {
            stacks[(mv.dest - 1) as usize].push(val);
        }
    }

    fn perform_moves_2(&mut self) {
        for mv in &self.moves {
            Cargo::perform_move_2(&mut self.stacks, mv);
        }
    }

    fn top(&self) -> String {
        let mut s = String::new();
        for stack in &self.stacks {
            s.push(*stack.last().unwrap());
        }
        s
    }
}

pub fn run1(input: &str) -> String {
    let mut cargo = Cargo::parse(input);
    cargo.perform_moves();
    cargo.top().to_string()
}

pub fn run2(input: &str) -> String {
    let mut cargo = Cargo::parse(input);
    cargo.perform_moves_2();
    cargo.top().to_string()
}
