#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum WinState {
    Loss,
    Tie,
    Win,
}

struct Round1 {
    move_opponent: Move,
    move_player: Move,
}

struct Round2 {
    move_opponent: Move,
    win_state: WinState,
}

impl Move {
    fn score(&self) -> u32 {
        match &self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn parse_opponent(input: &str) -> Move {
        match input {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!(),
        }
    }

    fn parse_player(input: &str) -> Move {
        match input {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!(),
        }
    }
}

impl WinState {
    fn score(&self) -> u32 {
        match &self {
            WinState::Loss => 0,
            WinState::Tie => 3,
            WinState::Win => 6,
        }
    }

    fn parse(input: &str) -> WinState {
        match input {
            "X" => WinState::Loss,
            "Y" => WinState::Tie,
            "Z" => WinState::Win,
            _ => panic!(),
        }
    }
}

impl Round1 {
    fn new(opponent: Move, player: Move) -> Round1 {
        Round1 {
            move_opponent: opponent,
            move_player: player,
        }
    }

    fn parse(input: &str) -> Vec<Round1> {
        let mut result = Vec::new();

        for line in input.lines() {
            let split: Vec<&str> = line.split(' ').collect();
            let opponent = Move::parse_opponent(split[0]);
            let player = Move::parse_player(split[1]);
            result.push(Round1::new(opponent, player));
        }

        result
    }

    fn state(&self) -> WinState {
        if &self.move_player == &self.move_opponent {
            WinState::Tie
        } else if &self.move_player == &Move::Rock && &self.move_opponent == &Move::Scissors {
            WinState::Win
        } else if &self.move_player == &Move::Paper && &self.move_opponent == &Move::Rock {
            WinState::Win
        } else if &self.move_player == &Move::Scissors && &self.move_opponent == &Move::Paper {
            WinState::Win
        } else {
            WinState::Loss
        }
    }

    fn score(&self) -> u32 {
        let move_score = &self.move_player.score();

        let win_score = &self.state().score();

        move_score + win_score
    }
}

impl Round2 {
    fn new(opponent: Move, state: WinState) -> Round2 {
        Round2 {
            move_opponent: opponent,
            win_state: state,
        }
    }

    fn parse(input: &str) -> Vec<Round2> {
        let mut result = Vec::new();

        for line in input.lines() {
            let split: Vec<&str> = line.split(' ').collect();
            let opponent = Move::parse_opponent(split[0]);
            let state = WinState::parse(split[1]);
            result.push(Round2::new(opponent, state));
        }

        result
    }

    fn move_player(&self) -> Move {
        if &self.win_state == &WinState::Tie {
            self.move_opponent
        } else if &self.win_state == &WinState::Win {
            if &self.move_opponent == &Move::Rock {
                Move::Paper
            } else if &self.move_opponent == &Move::Paper {
                Move::Scissors
            } else {
                Move::Rock
            }
        } else {
            if &self.move_opponent == &Move::Rock {
                Move::Scissors
            } else if &self.move_opponent == &Move::Paper {
                Move::Rock
            } else {
                Move::Paper
            }
        }
    }

    fn score(&self) -> u32 {
        let move_score = &self.move_player().score();

        let win_score = &self.win_state.score();

        move_score + win_score
    }
}

pub fn run1(input: &str) -> u32 {
    Round1::parse(input).iter().map(|x| x.score()).sum()
}

pub fn run2(input: &str) -> u32 {
    Round2::parse(input).iter().map(|x| x.score()).sum()
}
