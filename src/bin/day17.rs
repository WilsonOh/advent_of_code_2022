use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Left = -1,
    Right = 1,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RockType {
    Line,
    Plus,
    ReverseL,
    Stick,
    Square,
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| {
            if c == '<' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect_vec()
}

fn has_horizontal_collision(
    set: &BTreeSet<(i64, i64)>,
    rock_type: &RockType,
    new_pos: &(i64, i64),
) -> bool {
    match rock_type {
        RockType::Line => {
            return new_pos.1 < 0
                || new_pos.1 + 3 > 6
                || set.contains(&(new_pos.0, new_pos.1 + 3))
                || set.contains(new_pos)
        }
        RockType::Plus => {
            return new_pos.1 < 0
                || new_pos.1 + 2 > 6
                || set.contains(new_pos)
                || set.contains(&(new_pos.0, new_pos.1 + 2))
                || set.contains(&(new_pos.0 - 1, new_pos.1 + 1))
                || set.contains(&(new_pos.0 + 1, new_pos.1 + 1));
        }
        RockType::ReverseL => {
            let ans = new_pos.1 < 0
                || new_pos.1 + 2 > 6
                || set.contains(new_pos)
                || set.contains(&(new_pos.0, new_pos.1 + 2))
                || set.contains(&(new_pos.0 + 1, new_pos.1 + 2))
                || set.contains(&(new_pos.0 + 2, new_pos.1 + 2));
            ans
        }
        RockType::Stick => {
            return new_pos.1 < 0
                || new_pos.1 > 6
                || set.contains(new_pos)
                || set.contains(&(new_pos.0 + 1, new_pos.1))
                || set.contains(&(new_pos.0 + 2, new_pos.1))
                || set.contains(&(new_pos.0 + 3, new_pos.1));
        }
        RockType::Square => {
            return new_pos.1 < 0
                || new_pos.1 + 1 > 6
                || set.contains(new_pos)
                || set.contains(&(new_pos.0 + 1, new_pos.1))
                || set.contains(&(new_pos.0, new_pos.1 + 1))
                || set.contains(&(new_pos.0 + 1, new_pos.1 + 1));
        }
    }
}

fn has_vertical_collision(
    set: &BTreeSet<(i64, i64)>,
    rock_type: &RockType,
    new_pos: &(i64, i64),
) -> bool {
    match rock_type {
        RockType::Line => {
            return new_pos.0 < 0
                || set.contains(new_pos)
                || set.contains(&(new_pos.0, new_pos.1 + 1))
                || set.contains(&(new_pos.0, new_pos.1 + 2))
                || set.contains(&(new_pos.0, new_pos.1 + 3))
        }
        RockType::Plus => {
            return new_pos.0 < 0
                || set.contains(new_pos)
                || set.contains(&(new_pos.0, new_pos.1 + 2))
                || set.contains(&(new_pos.0 - 1, new_pos.1 + 1))
        }
        RockType::ReverseL => {
            return new_pos.0 < 0
                || set.contains(new_pos)
                || set.contains(&(new_pos.0, new_pos.1 + 1))
                || set.contains(&(new_pos.0, new_pos.1 + 2))
        }
        RockType::Stick => return new_pos.0 < 0 || set.contains(new_pos),
        RockType::Square => {
            return new_pos.0 < 0
                || set.contains(new_pos)
                || set.contains(&(new_pos.0, new_pos.1 + 1))
        }
    }
}

fn remove_rock_from_set(set: &mut BTreeSet<(i32, i32)>, rock_type: &RockType, pos: &(i32, i32)) {
    match rock_type {
        RockType::Line => {
            set.remove(pos);
            set.remove(&(pos.0, pos.1 + 1));
            set.remove(&(pos.0, pos.1 + 2));
            set.remove(&(pos.0, pos.1 + 3));
        }
        RockType::Plus => {
            set.remove(pos);
            set.remove(&(pos.0 + 1, pos.1 + 1));
            set.remove(&(pos.0, pos.1 + 2));
            set.remove(&(pos.0 - 1, pos.1 + 1));
        }
        RockType::ReverseL => {
            set.remove(pos);
            set.remove(&(pos.0, pos.1 + 1));
            set.remove(&(pos.0, pos.1 + 2));
            set.remove(&(pos.0 + 1, pos.1 + 2));
            set.remove(&(pos.0 + 2, pos.1 + 2));
        }
        RockType::Stick => {
            set.remove(pos);
            set.remove(&(pos.0 + 1, pos.1));
            set.remove(&(pos.0 + 2, pos.1));
            set.remove(&(pos.0 + 3, pos.1));
        }
        RockType::Square => {
            set.remove(pos);
            set.remove(&(pos.0, pos.1 + 1));
            set.remove(&(pos.0 + 1, pos.1));
            set.remove(&(pos.0 + 1, pos.1 + 1));
        }
    }
}

fn add_rock_to_set(set: &mut BTreeSet<(i64, i64)>, rock_type: &RockType, pos: &(i64, i64)) {
    match rock_type {
        RockType::Line => {
            set.insert(*pos);
            set.insert((pos.0, pos.1 + 1));
            set.insert((pos.0, pos.1 + 2));
            set.insert((pos.0, pos.1 + 3));
        }
        RockType::Plus => {
            set.insert(*pos);
            set.insert((pos.0 + 1, pos.1 + 1));
            set.insert((pos.0, pos.1 + 2));
            set.insert((pos.0 - 1, pos.1 + 1));
        }
        RockType::ReverseL => {
            set.insert(*pos);
            set.insert((pos.0, pos.1 + 1));
            set.insert((pos.0, pos.1 + 2));
            set.insert((pos.0 + 1, pos.1 + 2));
            set.insert((pos.0 + 2, pos.1 + 2));
        }
        RockType::Stick => {
            set.insert(*pos);
            set.insert((pos.0 + 1, pos.1));
            set.insert((pos.0 + 2, pos.1));
            set.insert((pos.0 + 3, pos.1));
        }
        RockType::Square => {
            set.insert(*pos);
            set.insert((pos.0, pos.1 + 1));
            set.insert((pos.0 + 1, pos.1));
            set.insert((pos.0 + 1, pos.1 + 1));
        }
    }
}

fn draw_grid(set: &BTreeSet<(i32, i32)>) {
    let highest = set.iter().next_back().unwrap().0 as usize;
    let mut grid = vec![vec!['.'; 7]; highest + 5];
    for (y, x) in set.iter() {
        if *y == -1 {
            continue;
        }
        grid[(highest + 4) - (*y as usize)][*x as usize] = '#';
    }
    for row in grid {
        for col in row {
            print!("{col}");
        }
        println!();
    }
}

fn part_one(dirs: &[Direction]) -> u32 {
    // (y, x)
    let mut set: BTreeSet<(i64, i64)> = BTreeSet::from([(-1, -1)]);
    let rock_types = vec![
        RockType::Line,
        RockType::Plus,
        RockType::ReverseL,
        RockType::Stick,
        RockType::Square,
    ];
    let mut dir_idx = 0;
    for i in 0..2023 {
        let curr_rock = &rock_types[i % rock_types.len()];
        let (highest, _) = set.iter().next_back().unwrap();
        let highest = *highest + 1;
        let mut curr_rock_pos = match curr_rock {
            RockType::Line => (highest + 3, 2),
            RockType::Plus => (highest + 4, 2),
            RockType::ReverseL => (highest + 3, 2),
            RockType::Stick => (highest + 3, 2),
            RockType::Square => (highest + 3, 2),
        };
        'inner: loop {
            let curr_dir = &dirs[dir_idx % (dirs.len() - 1)];
            let new_pos = (curr_rock_pos.0, curr_rock_pos.1 + *curr_dir as i64);
            dir_idx += 1;
            if !has_horizontal_collision(&set, curr_rock, &new_pos) {
                curr_rock_pos = new_pos;
            }
            let new_pos = (curr_rock_pos.0 - 1, curr_rock_pos.1);
            if !has_vertical_collision(&set, curr_rock, &new_pos) {
                curr_rock_pos = new_pos;
            } else {
                add_rock_to_set(&mut set, curr_rock, &curr_rock_pos);
                break 'inner;
            }
        }
    }
    set.remove(&(-1, -1));
    set.iter().next_back().unwrap().0 as u32 - 1
}

type Signature = (BTreeSet<(i64, i64)>, RockType, Direction);

fn get_deltas(set: &BTreeSet<(i64, i64)>, limit: i64) -> BTreeSet<(i64, i64)> {
    let (highest, _) = set.iter().next_back().unwrap();
    set.iter()
        .filter(|(y, _)| highest - y <= limit)
        .map(|(y, x)| (highest - y, *x))
        .collect()
}

fn part_two(dirs: &[Direction]) -> u32 {
    // (y, x)
    let mut set: BTreeSet<(i64, i64)> = BTreeSet::from([(-1, -1)]);
    let rock_types = vec![
        RockType::Line,
        RockType::Plus,
        RockType::ReverseL,
        RockType::Stick,
        RockType::Square,
    ];
    let mut added = 0;
    let mut dir_idx = 0;
    let mut cycle_check: HashMap<Signature, (i64, i64)> = HashMap::new();
    let mut i = 0u64;
    while i < (1e12) as u64 {
        let curr_rock = &rock_types[(i % rock_types.len() as u64) as usize];
        let (highest, _) = set.iter().next_back().unwrap();
        let highest = *highest + 1;
        let mut curr_rock_pos = match curr_rock {
            RockType::Line => (highest + 3, 2),
            RockType::Plus => (highest + 4, 2),
            RockType::ReverseL => (highest + 3, 2),
            RockType::Stick => (highest + 3, 2),
            RockType::Square => (highest + 3, 2),
        };
        'inner: loop {
            let curr_dir = &dirs[dir_idx % (dirs.len() - 1)];
            let new_pos = (curr_rock_pos.0, curr_rock_pos.1 + *curr_dir as i64);
            dir_idx += 1;
            if !has_horizontal_collision(&set, curr_rock, &new_pos) {
                curr_rock_pos = new_pos;
            }
            let new_pos = (curr_rock_pos.0 - 1, curr_rock_pos.1);
            if !has_vertical_collision(&set, curr_rock, &new_pos) {
                curr_rock_pos = new_pos;
            } else {
                add_rock_to_set(&mut set, curr_rock, &curr_rock_pos);
                let curr_state: Signature =
                    (get_deltas(&set, 30), curr_rock.clone(), curr_dir.clone());
                let (mut curr_top, _) = set.iter().next_back().unwrap();
                if cycle_check.contains_key(&curr_state) {
                    let (last_num_stones, last_top) = cycle_check[&curr_state];
                    let delta_top = curr_top - last_top;
                    let delta_stones = i as i64 - last_num_stones;
                    let amt = (1e12 as u64 - i) / delta_stones as u64;
                    added += amt * delta_top as u64;
                    i += amt * delta_stones as u64;
                }
                cycle_check.insert(curr_state, (i as i64, curr_top));
                break 'inner;
            }
        }
        i += 1;
    }
    set.remove(&(-1, -1));
    set.iter().next_back().unwrap().0 as u32 - 1 + added as u32
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let directions = parse_directions(&input);
    // let part_one_ans = part_one(&directions);
    let part_two_ans = part_two(&directions);
    // println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
