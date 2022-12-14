use anyhow::Result;
use itertools::Itertools;

fn parse_grid(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let coords_rows: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    coord
                        .split(",")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    for (x, y) in coords_rows.iter().flatten() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    let mut grid: Vec<Vec<bool>> = vec![vec![false; (max_x - min_x) + 1]; max_y + 1];
    for row in coords_rows {
        for coord in row.windows(2) {
            let (x1, y1) = coord[0];
            let (x2, y2) = coord[1];
            let x1 = x1 - min_x;
            let x2 = x2 - min_x;
            if x1 == x2 {
                if y1 < y2 {
                    for i in y1..=y2 {
                        grid[i][x1] = true;
                    }
                } else {
                    for i in (y2..=y1).rev() {
                        grid[i][x1] = true;
                    }
                }
            } else {
                if x1 < x2 {
                    for i in x1..=x2 {
                        grid[y1][i] = true;
                    }
                } else {
                    for i in (x2..=x1).rev() {
                        grid[y1][i] = true;
                    }
                }
            }
        }
    }
    let sand_point = (500 - min_x, 0);
    (grid, sand_point)
}

fn parse_grid_two(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let coords_rows: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    coord
                        .split(",")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    for (x, y) in coords_rows.iter().flatten() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 500]; max_y + 1];
    for row in coords_rows {
        for coord in row.windows(2) {
            let (x1, y1) = coord[0];
            let (x2, y2) = coord[1];
            let x1 = (x1 + min_x / 2) - min_x;
            let x2 = (x2 + min_x / 2) - min_x;
            if x1 == x2 {
                if y1 < y2 {
                    for i in y1..=y2 {
                        grid[i][x1] = true;
                    }
                } else {
                    for i in (y2..=y1).rev() {
                        grid[i][x1] = true;
                    }
                }
            } else {
                if x1 < x2 {
                    for i in x1..=x2 {
                        grid[y1][i] = true;
                    }
                } else {
                    for i in (x2..=x1).rev() {
                        grid[y1][i] = true;
                    }
                }
            }
        }
    }
    let sand_point = ((500 + min_x / 2) - min_x, 0);
    (grid, sand_point)
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for col in row {
            if *col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("\n\n");
}

fn part_one(input: &str) -> u32 {
    let (mut grid, sand_point) = parse_grid(&input);
    let mut ans = 0;
    let mut curr_pos = sand_point;
    'outer: loop {
        'inner: loop {
            let mut done = true;
            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                let nx = (curr_pos.0 as i32) + dx;
                let ny = (curr_pos.1 as i32) + dy;
                if (nx >= 0 && nx < grid[0].len() as i32) && (ny >= 0 && ny < grid.len() as i32) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !grid[ny][nx] {
                        curr_pos = (nx, ny);
                        done = false;
                        break;
                    }
                } else {
                    break 'outer;
                }
            }
            if done {
                ans += 1;
                grid[curr_pos.1][curr_pos.0] = true;
                curr_pos = sand_point;
                break 'inner;
            }
        }
    }
    ans
}

fn part_two(input: &str) -> u32 {
    let (mut grid, sand_point) = parse_grid_two(&input);
    let mut ans = 0;
    let mut curr_pos = sand_point;
    let mut extra_rows = vec![vec![false; grid[0].len()], vec![true; grid[0].len()]];
    grid.append(&mut extra_rows);
    'outer: loop {
        'inner: loop {
            let mut done = true;
            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                let nx = (curr_pos.0 as i32) + dx;
                let ny = (curr_pos.1 as i32) + dy;
                if (nx >= 0 && nx < grid[0].len() as i32) && (ny >= 0 && ny < grid.len() as i32) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !grid[ny][nx] {
                        if ny != grid.len() - 1 {
                            done = false;
                        }
                        curr_pos = (nx, ny);
                        break;
                    }
                }
            }
            if done {
                if curr_pos == sand_point {
                    ans += 1;
                    break 'outer;
                }
                ans += 1;
                grid[curr_pos.1][curr_pos.0] = true;
                curr_pos = sand_point;
                break 'inner;
            }
        }
    }
    ans
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = part_one(&input);
    let part_two_ans = part_two(&input);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
