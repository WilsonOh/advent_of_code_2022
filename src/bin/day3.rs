use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc: u32, curr| {
        let first: HashSet<char> = curr[..curr.len() / 2].chars().collect();
        let second: HashSet<char> = curr[curr.len() / 2..].chars().collect();
        let common_char = *first.intersection(&second).next().unwrap();
        let val = if common_char.is_lowercase() {
            (common_char as u8) - b'a' + 1
        } else {
            (common_char as u8) - b'A' + 27
        };
        acc + val as u32
    })
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .collect_vec()
        .chunks(3)
        .fold(0, |acc: u32, curr| {
            let first: HashSet<char> = curr[0].chars().collect();
            let second: HashSet<char> = curr[1].chars().collect();
            let third: HashSet<char> = curr[2].chars().collect();
            let tmp: HashSet<_> = first.intersection(&second).copied().collect();
            let common_char = *tmp.intersection(&third).next().unwrap();
            let val = if common_char.is_lowercase() {
                (common_char as u8) - b'a' + 1
            } else {
                (common_char as u8) - b'A' + 27
            };
            acc + val as u32
        })
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let ans = part_two(&input);
    println!("{ans}");
    Ok(())
}
