#![allow(non_snake_case, dead_code)]

use std::collections::HashMap;

/*
const EXAMPLE: &str = r#"A Y
B X
C Z"#;
*/

#[derive(Hash, PartialEq, Eq, Debug)]
enum State {
    Win,
    Lose,
    Draw,
}

fn get_match_result(a: &char, b: &char) -> State {
    if *a == *b {
        return State::Draw;
    }
    if *b == 'A' {
        if *a == 'B' {
            return State::Lose;
        }
        if *a == 'C' {
            return State::Win;
        }
    }
    if *b == 'B' {
        if *a == 'A' {
            return State::Win;
        }
        if *a == 'C' {
            return State::Lose;
        }
    }
    if *a == 'A' {
        return State::Lose;
    }
    return State::Win;
}

fn get_shape_needed(state: &State, a: &char) -> char {
    match state {
        State::Win => {
            if *a == 'A' {
                return 'B';
            }
            if *a == 'B' {
                return 'C';
            }
            return 'A';
        }
        State::Lose => {
            if *a == 'A' {
                return 'C';
            }
            if *a == 'B' {
                return 'A';
            }
            return 'B';
        }
        State::Draw => {
            return a.clone();
        }
    }
}

fn part_one(input: &str) -> u32 {
    let shape_score: HashMap<char, u32> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    let shape_map: HashMap<char, char> = HashMap::from([('X', 'A'), ('Y', 'B'), ('Z', 'C')]);
    let score_map: HashMap<State, u32> =
        HashMap::from([(State::Win, 6), (State::Draw, 3), (State::Lose, 0)]);
    input.lines().fold(0, |acc: u32, curr| {
        let mut tmp = curr.split_whitespace();
        let a = tmp.next().unwrap().chars().next().unwrap();
        let b = shape_map
            .get(&tmp.next().unwrap().chars().next().unwrap())
            .unwrap();
        acc + shape_score.get(&b).unwrap() + score_map.get(&get_match_result(&a, b)).unwrap()
    })
}

fn part_two(input: &str) -> u32 {
    let state_map: HashMap<char, State> =
        HashMap::from([('X', State::Lose), ('Y', State::Draw), ('Z', State::Win)]);
    let score_map: HashMap<State, u32> =
        HashMap::from([(State::Win, 6), (State::Draw, 3), (State::Lose, 0)]);
    let shape_score: HashMap<char, u32> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    input.lines().fold(0, |acc: u32, curr| {
        let mut tmp = curr.split_whitespace();
        let a = tmp.next().unwrap().chars().next().unwrap();
        let b = tmp.next().unwrap().chars().next().unwrap();
        let state = state_map.get(&b).unwrap();
        let shape = get_shape_needed(state, &a);
        acc + shape_score.get(&shape).unwrap() + score_map.get(state).unwrap()
    })
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let ans = part_two(&input);
    println!("{ans}");
    Ok(())
}
