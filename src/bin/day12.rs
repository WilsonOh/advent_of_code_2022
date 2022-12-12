use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;

fn bfs_get_path_len(grid: &[Vec<u32>], start: (usize, usize), end: (usize, usize)) -> u32 {
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut q: VecDeque<(usize, usize)> = VecDeque::from([start]);

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    dist[start.0][start.1] = 0;
    visited[start.0][start.1] = true;

    while let Some((row, col)) = q.pop_front() {
        if (row, col) == end {
            return dist[row][col];
        }
        for (dr, dc) in dirs {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if (nr >= 0 && nr < grid.len() as i32) && (nc >= 0 && nc < grid[0].len() as i32) {
                let nr = nr as usize;
                let nc = nc as usize;
                if ((grid[nr][nc] < grid[row][col]) || (grid[nr][nc] - grid[row][col] <= 1))
                    && !visited[nr][nc]
                {
                    visited[nr][nc] = true;
                    dist[nr][nc] = dist[nr][nc].min(dist[row][col] + 1);
                    q.push_back((nr, nc));
                }
            }
        }
    }
    u32::MAX
}

fn part_one(input: &str) -> u32 {
    let grid = parse_grid(&input);
    let (start, end) = get_start_and_end_pos(&input);
    bfs_get_path_len(&grid, start, end)
}

fn part_two(input: &str) -> u32 {
    let grid = parse_grid(&input);
    let (_, end) = get_start_and_end_pos(&input);
    let a_pos = get_a_pos(&input);
    a_pos
        .iter()
        .map(|start| bfs_get_path_len(&grid, *start, end))
        .min()
        .unwrap() as u32
}

fn get_a_pos(input: &str) -> Vec<(usize, usize)> {
    let mut pos: Vec<(usize, usize)> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' || ch == 'a' {
                pos.push((row, col));
            }
        }
    }
    pos
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
    let part_one_ans = part_one(&input);
    let part_two_ans = part_two(&input);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
