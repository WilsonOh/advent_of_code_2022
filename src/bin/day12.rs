use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;

fn part_one(input: &str) -> u32 {
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (start, end) = get_start_and_end_pos(&input);
    let grid = parse_grid(&input);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut q: VecDeque<(usize, usize)> = VecDeque::from([start]);
    let mut parent = vec![vec![(-1, -1); grid[0].len()]; grid.len()];
    while let Some((row, col)) = q.pop_front() {
        if (row, col) == end {
            break;
        }
        visited[row][col] = true;
        for (dr, dc) in dirs {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if (nr >= 0 && nr < grid.len() as i32) && (nc >= 0 && nc < grid[0].len() as i32) {
                let nr = nr as usize;
                let nc = nc as usize;
                if ((grid[nr][nc] < grid[row][col]) || (grid[nr][nc] - grid[row][col] <= 1))
                    && !visited[nr][nc]
                {
                    if parent[nr][nc] == (-1, -1) {
                        parent[nr][nc] = (row as i32, col as i32);
                    }
                    q.push_back((nr, nc));
                }
            }
        }
    }
    let mut curr = (end.0 as i32, end.1 as i32);
    let mut ans = 0;
    while curr != (-1, -1) {
        ans += 1;
        curr = parent[curr.0 as usize][curr.1 as usize];
    }
    ans - 1
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch == 'S' {
                        return 0;
                    }
                    if ch == 'E' {
                        return 25;
                    }
                    return (ch as u32) - ('a' as u32);
                })
                .collect_vec()
        })
        .collect_vec()
}

fn get_start_and_end_pos(input: &str) -> ((usize, usize), (usize, usize)) {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_pos = (row, col);
            }
            if ch == 'E' {
                end_pos = (row, col);
            }
        }
    }
    (start_pos, end_pos)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("{}", part_one(&input));
    Ok(())
}
