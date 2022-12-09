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

fn tail_is_pulled(head_coords: &Coords, tail_coords: &Coords) -> bool {
    let dist = (((head_coords.row - tail_coords.row).pow(2)
        + (head_coords.col - tail_coords.col).pow(2)) as f64)
        .sqrt();
    dist.floor() as i32 >= 2
}

fn get_new_tail_coords(head_move: &Move, head_coords: &Coords) -> Coords {
    let (dr, dc) = head_move.dir;
    Coords {
        row: head_coords.row - dr,
        col: head_coords.col - dc,
    }
}

fn print_grid(knots: &[Coords]) {
    println!("{knots:?}");
    let mut grid = [['.'; 6]; 5];
    grid[0][0] = 's';
    for (idx, knot) in knots.iter().rev().enumerate() {
        let mut c = char::from_digit(idx as u32, 10).unwrap();
        if idx == 0 {
            c = 'H';
        }
        grid[knot.row as usize][knot.col as usize] = c;
    }
    for row in grid {
        println!("{row:?}");
    }
}

fn part_one(moves: &[Move]) -> u32 {
    let mut tail_coords = Coords::default();
    let mut head_coords = Coords::default();
    let mut ans = 1u32;
    let mut visited: HashSet<Coords> = HashSet::new();
    visited.insert(tail_coords);
    for m in moves {
        let (dr, dc) = m.dir;
        for _ in 0..m.mag {
            head_coords.row += dr;
            head_coords.col += dc;
            if tail_is_pulled(&head_coords, &tail_coords) {
                tail_coords = get_new_tail_coords(m, &head_coords);
                if !visited.contains(&tail_coords) {
                    visited.insert(tail_coords);
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn part_two(moves: &[Move]) -> u32 {
    let mut knots = [Coords::default(); 10];
    let mut ans = 1u32;
    let mut visited: HashSet<Coords> = HashSet::new();
    visited.insert(knots[0]);
    for m in moves {
        let (dr, dc) = m.dir;
        for _ in 0..m.mag {
            knots[0].row += dr;
            knots[0].col += dc;
            for i in 1..=9 {
                if tail_is_pulled(&knots[i - 1], &knots[i]) {
                    knots[i] = get_new_tail_coords(m, &knots[i - 1]);
                    if i == 9 && !visited.contains(&knots[i]) {
                        visited.insert(knots[i]);
                        ans += 1;
                    }
                }
            }
            println!("{knots:?}\n");
        }
    }
    ans
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("sample.txt")?;
    let moves = parse_moves(&input);
    let part_one_ans = part_one(&moves);
    let part_two_ans = part_two(&moves);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
