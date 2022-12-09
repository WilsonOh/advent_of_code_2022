use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Move {
    mag: u32,
    dir: (i32, i32),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
struct Coords {
    row: i32,
    col: i32,
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (dir, mag) = line.split_whitespace().collect_tuple().unwrap();
            let mag: u32 = mag.parse().unwrap();
            match dir {
                "R" => Move { mag, dir: (0, 1) },
                "L" => Move { mag, dir: (0, -1) },
                "U" => Move { mag, dir: (1, 0) },
                "D" => Move { mag, dir: (-1, 0) },
                _ => panic!("invalid direction"),
            }
        })
        .collect_vec()
}

/// Check whether the tail is being pulled, i.e. the tail is 2 units or more
/// away from the head in any direction by calculating the euclidian distance
/// between the tail and head using the standard formula
fn tail_is_pulled(head_coords: &Coords, tail_coords: &Coords) -> bool {
    let dist = (((head_coords.row - tail_coords.row).pow(2)
        + (head_coords.col - tail_coords.col).pow(2)) as f64)
        .sqrt();
    dist >= 2.0
}

fn normalize_coord(num: &i32) -> i32 {
    num.checked_div(num.abs()).unwrap_or(*num)
}

/// # Observation: we can get the new coordinates of the tail by simply getting
/// # the "coord difference" between head and tail.
///
/// The sign of the row and col in the coord difference will tell us how to update
/// the tail coords.
///
/// E.g. coord difference = (-dr, -dc) -> tail moves diagonally down-left
///      coord difference = (dr, 0) -> tail moves upwards
///      etc...
///
/// Since we know that the tail will only shift **one** unit in any of the 8 directions
/// in any one move, we can simply "normalize" the coord difference to get the unit value
/// we need for updating the tail coords, be it in the negative or positive direction.
fn get_new_tail_coords(tail_coords: &Coords, head_coords: &Coords) -> Coords {
    let dr = normalize_coord(&(head_coords.row - tail_coords.row));
    let dc = normalize_coord(&(head_coords.col - tail_coords.col));
    Coords {
        row: tail_coords.row + dr,
        col: tail_coords.col + dc,
    }
}

/// Generalized function to solve Day 9 for n knots
fn knots(moves: &[Move], num_knots: usize) -> u32 {
    let mut knots = vec![Coords::default(); num_knots];
    let mut visited: HashSet<Coords> = HashSet::new();
    visited.insert(knots[0]);
    moves.iter().for_each(|m| {
        let (dr, dc) = m.dir;
        for _ in 0..m.mag {
            knots[0].row += dr;
            knots[0].col += dc;
            for i in 1..num_knots {
                // Compare each tail knot with the knot directly in front of it
                if tail_is_pulled(&knots[i - 1], &knots[i]) {
                    knots[i] = get_new_tail_coords(&knots[i], &knots[i - 1]);
                    // only insert the path of the TAIL knot,
                    // HashSet will take care of duplicates for us
                    if i == num_knots - 1 {
                        visited.insert(knots[i]);
                    }
                }
            }
        }
    });
    // The size of the HashSet is basically the number of coords that
    // TAIL has been to
    visited.len() as u32
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let moves = parse_moves(&input);
    let part_one_ans = knots(&moves, 2);
    let part_two_ans = knots(&moves, 10);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
