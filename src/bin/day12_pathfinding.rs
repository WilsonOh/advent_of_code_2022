use anyhow::Result;
use pathfinding::prelude::{bfs, Matrix};

fn part_one(input: &str) -> u32 {
    let (ref m, s, e) = parse_into_matrix(input);
    bfs(
        &s,
        |&node| {
            m.neighbours(node, false)
                .filter(move |&idx| m[idx] <= m[node] + 1)
        },
        |&node| node == e,
    )
    .unwrap()
    .len() as u32
        - 1
}

fn part_two(input: &str) -> u32 {
    let (ref m, _, e) = parse_into_matrix(input);
    bfs(
        &e,
        |&node| {
            m.neighbours(node, false)
                .filter(move |&idx| m[node] <= m[idx] + 1)
        },
        |&node| m[node] == b'a',
    )
    .unwrap()
    .len() as u32
        - 1
}

fn parse_into_matrix(input: &str) -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let mut m = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let start = m.indices().find(|&idx| m[idx] == b'S').unwrap();
    let end = m.indices().find(|&idx| m[idx] == b'E').unwrap();
    m[start] = b'a';
    m[end] = b'z';
    (m, start, end)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
    Ok(())
}
