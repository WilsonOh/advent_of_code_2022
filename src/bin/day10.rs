use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|line| {
            let tokens = line.split_whitespace().collect_vec();
            match tokens[0] {
                "addx" => vec![
                    Instruction::Noop,
                    Instruction::Addx(tokens[1].parse().unwrap()),
                ],
                "noop" => vec![Instruction::Noop],
                _ => panic!("Invalid instruction"),
            }
        })
        .collect_vec()
}

fn get_pixel_positions_for_each_cycle(instructions: &[Instruction]) -> Vec<i32> {
    let mut x: i32 = 1;
    instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Noop => x,
            Instruction::Addx(val) => {
                let ret = x;
                x += val;
                ret
            }
        })
        .collect_vec()
}

fn part_one(instructions: &[Instruction]) -> i32 {
    let pixel_pos = get_pixel_positions_for_each_cycle(instructions);
    let mut ans = 0;
    for cycle in 1..=240 {
        if (cycle + 20) % 40 == 0 {
            ans += cycle as i32 * pixel_pos[cycle - 1];
        }
    }
    ans
}

fn print_grid(grid: &[[char; 40]]) {
    for row in grid {
        for col in row {
            print!("{col}");
        }
        println!();
    }
}

fn part_two(instructions: &[Instruction]) {
    let pixel_pos = get_pixel_positions_for_each_cycle(instructions);
    let mut grid = [[' '; 40]; 6];
    for (row_idx, row) in pixel_pos.chunks(40).enumerate() {
        for (col_idx, pos) in row.iter().enumerate() {
            if (pos - 1..=pos + 1).contains(&(col_idx as i32)) {
                grid[row_idx][col_idx] = '#';
            }
        }
    }
    print_grid(&grid);
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let instructions = parse_input(&input);
    let part_one_ans = part_one(&instructions);
    println!("part one: {part_one_ans}");
    println!("part two:");
    part_two(&instructions);
    Ok(())
}
