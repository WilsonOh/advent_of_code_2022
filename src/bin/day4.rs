#![allow(unused)]

use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::fs;

type Section = (u32, u32, u32, u32);

fn get_range(pair: &str) -> (u32, u32) {
    pair.split('-')
        .filter_map(|num| num.parse::<u32>().ok())
        .collect_tuple()
        .unwrap()
}

fn get_ranges_fmt(input: &str) -> Vec<Section> {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap())
        .collect()
}

fn part_one(sections: &[Section]) -> u32 {
    sections
        .iter()
        .filter(|section| {
            let (a, b, c, d) = section;
            (c >= a && d <= b) || (a >= c && b <= d)
        })
        .count() as u32
}

fn part_two(sections: &[Section]) -> u32 {
    sections
        .iter()
        .filter(|section| {
            let (a, b, c, d) = section;
            (a <= c && c <= b) || (c <= a && a <= d)
        })
        .count() as u32
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let ranges = get_ranges_fmt(&input);
    let ans = part_two(&ranges);
    println!("{ans}");
    Ok(())
}
