use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn get_range(pair: &str) -> (u32, u32) {
    pair.split("-")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect_tuple()
        .unwrap()
}

fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc, curr| {
        let mut overlaps: bool = false;
        let pairs = curr.split(",").collect::<Vec<_>>();
        let (a, b) = get_range(pairs[0]);
        let (c, d) = get_range(pairs[1]);
        if (b - a) > (d - c) {
            if c >= a && d <= b {
                overlaps = true;
            }
        } else {
            if a >= c && b <= d {
                overlaps = true;
            }
        }
        acc + if overlaps { 1 } else { 0 }
    })
}

fn part_two(input: &str) -> u32 {
    input.lines().fold(0, |acc, curr| {
        let mut overlaps: bool = true;
        let pairs = curr.split(",").collect::<Vec<_>>();
        let (a, b) = get_range(pairs[0]);
        let (c, d) = get_range(pairs[1]);
        if a < c {
            if b < c {
                overlaps = false;
            }
        } else {
            if d < a {
                overlaps = false;
            }
        }
        acc + if overlaps { 1 } else { 0 }
    })
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let ans = part_two(&input);
    println!("{ans}");
    Ok(())
}
