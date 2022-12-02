use std::{cmp::Ordering, str::FromStr};

enum State {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("invalid move".to_string()),
        }
    }
}

impl FromStr for State {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(State::Lose),
            "Y" => Ok(State::Draw),
            "Z" => Ok(State::Win),
            _ => Err("invalid move".to_string()),
        }
    }
}

impl PartialOrd for Move {
    /// Use some math trick to easily calculate who wins
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if *self as u8 == *other as u8 {
            return Some(Ordering::Equal);
        }
        if (*self as u8 + 1) % 3 == *other as u8 {
            return Some(Ordering::Less);
        }
        Some(Ordering::Greater)
    }
}

fn get_shape_needed(state: &State, enemy_move: &Move) -> Move {
    match state {
        State::Win => match enemy_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        State::Lose => match enemy_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        State::Draw => enemy_move.clone(),
    }
}

fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc: u32, curr| {
        let tmp: Vec<Move> = curr
            .split_whitespace()
            .map(|s| s.parse::<Move>().unwrap())
            .collect();
        acc + match tmp[0].partial_cmp(&tmp[1]) {
            Some(Ordering::Less) => State::Win,
            Some(Ordering::Equal) => State::Draw,
            Some(Ordering::Greater) => State::Lose,
            None => unreachable!(),
        } as u32
            + tmp[1] as u32
            + 1 // add one since the problem states that the moves are 1-indexed
    })
}

fn part_two(input: &str) -> u32 {
    input.lines().fold(0, |acc: u32, curr| {
        let tmp: Vec<&str> = curr.split_whitespace().collect();
        let enemy_move: Move = tmp[0].parse().unwrap();
        let state: State = tmp[1].parse().unwrap();
        let my_move = get_shape_needed(&state, &enemy_move);
        acc + my_move as u32 + state as u32 + 1
    })
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let ans = part_one(&input);
    println!("{ans}");
    Ok(())
}
