use std::str::FromStr;

use crate::helpers::read_lines;

pub fn test_day2() {
    println!("Running day 2!");
    run();
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn points(self) -> i32 {
        match self {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(String::from("Invalid String")),
        }
    }
}


#[derive(Debug)]
enum RockPaperScisors {
    Rock,
    Paper,
    Scisors,
}

impl FromStr for RockPaperScisors {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RockPaperScisors::Rock),
            "B" | "Y" => Ok(RockPaperScisors::Paper),
            "C" | "Z" => Ok(RockPaperScisors::Scisors),
            _ => Err(String::from("Invalid String")),
        }
    }
}

impl RockPaperScisors {
    fn beats(&self, other: RockPaperScisors) -> GameResult {
        match self {
            RockPaperScisors::Rock => match other {
                RockPaperScisors::Rock => GameResult::Draw,
                RockPaperScisors::Paper => GameResult::Lose,
                RockPaperScisors::Scisors => GameResult::Win,
            },
            RockPaperScisors::Paper => match other {
                RockPaperScisors::Rock => GameResult::Win,
                RockPaperScisors::Paper => GameResult::Draw,
                RockPaperScisors::Scisors => GameResult::Lose,
            },
            RockPaperScisors::Scisors => match other {
                RockPaperScisors::Rock => GameResult::Lose,
                RockPaperScisors::Paper => GameResult::Win,
                RockPaperScisors::Scisors => GameResult::Draw,
            },
        }
    }

    fn points(&self) -> i32 {
        match self {
            RockPaperScisors::Rock => 1,
            RockPaperScisors::Paper => 2,
            RockPaperScisors::Scisors => 3,
        }
    }

    fn new(opponent: &RockPaperScisors, result: GameResult) -> Self {
        match opponent {
            RockPaperScisors::Rock => match result {
                GameResult::Lose => RockPaperScisors::Scisors,
                GameResult::Draw => RockPaperScisors::Rock,
                GameResult::Win => RockPaperScisors::Paper,
            },
            RockPaperScisors::Paper => match result {
                GameResult::Lose => RockPaperScisors::Rock,
                GameResult::Draw => RockPaperScisors::Paper,
                GameResult::Win => RockPaperScisors::Scisors,
            },
            RockPaperScisors::Scisors => match result {
                GameResult::Lose => RockPaperScisors::Paper,
                GameResult::Draw => RockPaperScisors::Scisors,
                GameResult::Win => RockPaperScisors::Rock,
            },
        }
    }
}


fn parse_game(s: &str) -> Result<(RockPaperScisors, RockPaperScisors), String> {
    let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
    if tokens.len() != 2 {
        return Err(String::from("Bad number of tokens"));
    }

    let player = tokens[1].parse::<RockPaperScisors>()?;
    let opponent = tokens[0].parse::<RockPaperScisors>()?;

    Ok((player, opponent))
}

fn parse_inverse_game(s: &str) -> Result<(RockPaperScisors, RockPaperScisors), String> {
    let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
    if tokens.len() != 2 {
        return Err(String::from("Bad number of tokens"));
    }

    let opponent = tokens[0].parse::<RockPaperScisors>()?;
    let desired_result = tokens[1].parse::<GameResult>()?;

    Ok((RockPaperScisors::new(&opponent, desired_result), opponent))
}

fn game_result(game: (RockPaperScisors, RockPaperScisors)) -> i32 {
    game.0.points() + game.0.beats(game.1).points()
}

fn run() {
    let Ok(lines) = read_lines("./data/day2.txt") else {
        println!("Error! couldn't read file");
        return;
    };

    let score = lines
        .filter_map(|l| l.ok())
        .map(|s| parse_game(&s))
        .filter_map(|r| r.ok())
        .map(game_result)
        .sum::<i32>();

    println!("Total score: {}", score);

    let Ok(lines) = read_lines("./data/day2.txt") else {
        println!("Error! couldn't read file");
        return;
    };
    let score = lines
        .filter_map(|l| l.ok())
        .map(|s| parse_inverse_game(&s))
        .filter_map(|r| r.ok())
        .map(game_result)
        .sum::<i32>();

    println!("Inverted score: {}", score);
}
