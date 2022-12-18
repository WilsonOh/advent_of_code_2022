use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vec3(i64, i64, i64);

fn parse_coords(input: &str) -> HashSet<Vec3> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(",")
                .filter_map(|c| c.parse::<i64>().ok())
                .collect_tuple()
                .unwrap();
            Vec3(x, y, z)
        })
        .collect()
}

fn reaches_outside(coords: &HashSet<Vec3>, curr: Vec3) -> bool {
    let mut q: VecDeque<Vec3> = VecDeque::from([curr]);
    let mut visited: HashSet<Vec3> = HashSet::from_iter(coords.iter().copied());
    while let Some(curr) = q.pop_front() {
        let Vec3(x, y, z) = curr;
        if !visited.contains(&curr) {
            visited.insert(curr);
            if !(&[x, y, z]).iter().all(|&c| c >= 0 && c <= 19) {
                return true;
            }
            for delta in [-1, 1] {
                q.push_back(Vec3(x + delta, y, z));
                q.push_back(Vec3(x, y + delta, z));
                q.push_back(Vec3(x, y, z + delta));
            }
        }
    }
    false
}

fn part_one(coords: &mut HashSet<Vec3>) -> u64 {
    let mut total_faces = 6 * coords.len();
    for Vec3(x, y, z) in coords.iter() {
        for delta in [-1, 1] {
            if coords.contains(&Vec3(*x + delta, *y, *z)) {
                total_faces -= 1;
            }
            if coords.contains(&Vec3(*x, *y + delta, *z)) {
                total_faces -= 1;
            }
            if coords.contains(&Vec3(*x, *y, *z + delta)) {
                total_faces -= 1;
            }
        }
    }
    total_faces as u64
}

fn part_two(coords: &mut HashSet<Vec3>) -> u64 {
    let mut total_faces = 6 * coords.len();
    for Vec3(x, y, z) in coords.iter() {
        for delta in [-1, 1] {
            if !reaches_outside(coords, Vec3(*x + delta, *y, *z)) {
                total_faces -= 1;
            }
            if !reaches_outside(coords, Vec3(*x, *y + delta, *z)) {
                total_faces -= 1;
            }
            if !reaches_outside(coords, Vec3(*x, *y, *z + delta)) {
                total_faces -= 1;
            }
        }
    }
    total_faces as u64
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut coords = parse_coords(&input);
    let part_one_ans = part_one(&mut coords);
    let part_two_ans = part_two(&mut coords);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
