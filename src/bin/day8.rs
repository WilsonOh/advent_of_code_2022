use anyhow::Result;
use itertools::Itertools;

fn find_col_view(
    grid: &[Vec<u32>],
    col: usize,
    start_row: usize,
    end_row: usize,
    curr: u32,
) -> u32 {
    let mut ans = 0u32;
    let range: Box<dyn Iterator<Item = usize>> = if start_row > end_row {
        Box::new((end_row..start_row).rev())
    } else {
        Box::new(start_row..end_row)
    };
    for i in range {
        if grid[i][col] < curr {
            ans += 1;
        } else {
            return ans;
        }
    }
    ans
}

fn find_row_view(
    grid: &[Vec<u32>],
    row: usize,
    start_col: usize,
    end_col: usize,
    curr: u32,
) -> u32 {
    let mut ans = 0u32;
    let range: Box<dyn Iterator<Item = usize>> = if start_col > end_col {
        Box::new((end_col..start_col).rev())
    } else {
        Box::new(start_col..end_col)
    };
    for i in range {
        if grid[row][i] < curr {
            ans += 1;
        } else {
            return ans;
        }
    }
    ans
}

fn check_col_visible(
    grid: &[Vec<u32>],
    col: usize,
    start_row: usize,
    end_row: usize,
    curr: u32,
) -> bool {
    find_col_view(grid, col, start_row, end_row, curr) == (end_row - start_row) as u32
}

fn check_row_visible(
    grid: &[Vec<u32>],
    row: usize,
    start_col: usize,
    end_col: usize,
    curr: u32,
) -> bool {
    find_row_view(grid, row, start_col, end_col, curr) == (end_col - start_col) as u32
}

fn is_visible(grid: &[Vec<u32>], row: usize, col: usize) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let curr = grid[row][col];
    check_col_visible(grid, col, 0, row, curr)
        || check_col_visible(grid, col, row + 1, m, curr)
        || check_row_visible(grid, row, 0, col, curr)
        || check_row_visible(grid, row, col + 1, n, curr)
}

fn calculate_scenic_score(grid: &[Vec<u32>], row: usize, col: usize) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let curr = grid[row][col];
    // WARN: PLease don't look here. This is my last ditch effort to make my code "general"
    // so that in can be used in both part one and two...FML

    // Only add 1 when the the curr spot is not visible in the current direction (very hacky...)
    let i = find_row_view(grid, row, col, 0, curr)
        + if check_row_visible(grid, row, 0, col, curr) {
            0
        } else {
            1
        };
    let j = find_row_view(grid, row, col + 1, n, curr)
        + if check_row_visible(grid, row, col + 1, n, curr) {
            0
        } else {
            1
        };
    let k = find_col_view(grid, col, row, 0, curr)
        + if check_col_visible(grid, col, 0, row, curr) {
            0
        } else {
            1
        };
    let l = find_col_view(grid, col, row + 1, m, curr)
        + if check_col_visible(grid, col, row + 1, m, curr) {
            0
        } else {
            1
        };
    i * j * k * l
}

fn part_two(grid: &[Vec<u32>]) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0u32;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let curr = calculate_scenic_score(grid, i, j);
            ans = ans.max(curr);
        }
    }
    ans
}

fn part_one(grid: &[Vec<u32>]) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans: u32 = 0;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if is_visible(grid, i, j) {
                ans += 1;
            }
        }
    }
    ans + ((2 * m) + (2 * n) - 4) as u32
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect_vec())
        .collect_vec()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let grid = parse_grid(&input);
    let part_one_ans = part_one(&grid);
    let part_two_ans = part_two(&grid);
    println!("part_one = {part_one_ans}");
    println!("part_two = {part_two_ans}");
    Ok(())
}
