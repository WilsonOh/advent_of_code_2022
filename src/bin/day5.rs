use std::collections::VecDeque;

use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    size: usize,
}

/// parse the input into a Vec of stacks representing the stacks of cargo in the question,
/// and also a Vec of moves
fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut ret = vec![];
    let mut lines = input.lines();
    lines
        .by_ref()
        // Basically iterate every line until we hit a digit (end of the stacks)
        .take_while(|line| !line.chars().nth(1).unwrap().is_digit(10))
        .for_each(|line| {
            // initialize the stacks upon first run
            if ret.is_empty() {
                ret.resize((line.len() as f64 / 4.0).ceil() as usize, VecDeque::new());
            }
            let c = line.chars().collect_vec();
            let c = c
                // each chunk is represents an item on the stack
                .chunks(4)
                .map(|chunk| chunk.iter().collect::<String>().trim().to_string())
                .collect_vec();
            for (idx, chunk) in c.iter().enumerate() {
                if !chunk.is_empty() {
                    let val = scan_fmt!(chunk, "[{}]", char).unwrap();
                    ret[idx].push_front(val);
                }
            }
        });
    lines.next();
    let moves = lines
        .map(|line| {
            let (size, from, to) =
                scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
            Move {
                size,
                from: (from - 1),
                to: (to - 1),
            }
        })
        .collect_vec();
    (ret, moves)
}

fn part_one(input: &str) -> String {
    let (mut stacks, moves) = parse_input(&input);
    for m in moves {
        for _ in 0..m.size {
            let to_ins = stacks[m.from].pop_back().unwrap();
            stacks[m.to].push_back(to_ins);
        }
    }
    stacks.iter().map(|stack| stack.back().unwrap()).collect()
}

fn part_two(input: &str) -> String {
    let (mut stacks, moves) = parse_input(&input);
    for m in moves {
        let idx = stacks[m.from].len() - m.size;
        let mut to_ins: VecDeque<_> = stacks[m.from].drain(idx..).collect();
        stacks[m.to].append(&mut to_ins);
    }
    stacks.iter().map(|stack| stack.back().unwrap()).collect()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let ans = part_two(&input);
    println!("{ans}");
    Ok(())
}
