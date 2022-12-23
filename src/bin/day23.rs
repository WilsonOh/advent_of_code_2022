use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn has_adjacent_elves(elves: &HashSet<(i32, i32)>, curr_pos: &(i32, i32)) -> bool {
    let (r, c) = curr_pos;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r + dr;
            let nc = c + dc;
            if elves.contains(&(nr, nc)) {
                return true;
            }
        }
    }
    false
}

type DirectionList = [([(i32, i32); 3], (i32, i32)); 4];

fn propose_new_direction(
    elves: &HashSet<(i32, i32)>,
    curr_pos: &(i32, i32),
    direction_list: &DirectionList,
) -> Option<(i32, i32)> {
    let (r, c) = curr_pos;
    for (to_check, to_go) in direction_list {
        let mut can_go = true;
        for (dr, dc) in to_check {
            let nr = r + dr;
            let nc = c + dc;
            if elves.contains(&(nr, nc)) {
                can_go = false;
                break;
            }
        }
        if can_go {
            return Some((to_go.0 + r, to_go.1 + c));
        }
    }
    None
}

fn part_one(input: &str) -> u32 {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            if item == '#' {
                elves.insert((row as i32, col as i32));
            }
        }
    }
    let mut direction_list: DirectionList = [
        ([(-1, 0), (-1, -1), (-1, 1)], (-1, 0)),
        ([(1, 0), (1, 1), (1, -1)], (1, 0)),
        ([(0, -1), (-1, -1), (1, -1)], (0, -1)),
        ([(0, 1), (1, 1), (-1, 1)], (0, 1)),
    ];
    for _ in 0..=10 {
        // first half of round
        let mut next_pos: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for elf in elves.iter() {
            if has_adjacent_elves(&elves, elf) {
                if let Some(to_go) = propose_new_direction(&elves, elf, &direction_list) {
                    next_pos.entry(to_go).or_default().push(*elf);
                }
            }
        }
        // second half of round
        for (new_pos, old_pos) in next_pos.iter() {
            if old_pos.len() == 1 {
                elves.remove(&(old_pos[0]));
                elves.insert(*new_pos);
            }
        }
        direction_list.rotate_left(1);
    }
    let left_bound = &elves.iter().map(|(_, c)| *c).min().unwrap();
    let right_bound = &elves.iter().map(|(_, c)| *c).max().unwrap();
    let top_bound = &elves.iter().map(|(r, _)| *r).min().unwrap();
    let btm_bound = &elves.iter().map(|(r, _)| *r).max().unwrap();
    let width = (right_bound - left_bound) + 1;
    let height = (btm_bound - top_bound) + 1;
    let area = width * height;
    (area - elves.len() as i32) as u32
}

fn part_two(input: &str) -> u32 {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            if item == '#' {
                elves.insert((row as i32, col as i32));
            }
        }
    }
    let mut direction_list: DirectionList = [
        ([(-1, 0), (-1, -1), (-1, 1)], (-1, 0)),
        ([(1, 0), (1, 1), (1, -1)], (1, 0)),
        ([(0, -1), (-1, -1), (1, -1)], (0, -1)),
        ([(0, 1), (1, 1), (-1, 1)], (0, 1)),
    ];
    let mut next_pos: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for round in 1.. {
        // first half of round
        for elf in elves.iter() {
            if has_adjacent_elves(&elves, elf) {
                if let Some(to_go) = propose_new_direction(&elves, elf, &direction_list) {
                    next_pos.entry(to_go).or_default().push(*elf);
                }
            }
        }
        // no more movements
        if next_pos.is_empty() {
            return round;
        }
        // second half of round
        for (new_pos, old_pos) in next_pos.iter() {
            if old_pos.len() == 1 {
                elves.remove(&(old_pos[0]));
                elves.insert(*new_pos);
            }
        }
        next_pos.clear();
        direction_list.rotate_left(1);
    }
    unreachable!("uhh this shouldn't happen")
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = part_one(&input);
    println!("part one: {part_one_ans}");
    let part_two_ans = part_two(&input);
    println!("part two: {part_two_ans}");
    Ok(())
}
