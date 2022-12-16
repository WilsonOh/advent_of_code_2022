use anyhow::Result;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

fn _foo(set: &HashSet<(i32, i32)>) {
    let mut grid = [['.'; 100]; 100];
    for (x, y) in set {
        println!("x={x}, y={y}");
        grid[(*y + 20) as usize][(*x + 20) as usize] = '#';
    }
    for row in grid {
        for col in row {
            print!("{col}");
        }
        println!();
    }
}

fn parse_input(input: &str) -> (HashSet<(i32, i32, i32)>, HashSet<(i32, i32)>) {
    let mut sources: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();
    for line in input.lines() {
        let (sx, sy, bx, by) = scan_fmt!(
            line,
            "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as i32;
        sources.insert((sx, sy, dist));
        beacons.insert((bx, by));
    }
    (sources, beacons)
}

fn is_valid_pos(sources: &HashSet<(i32, i32, i32)>, x: i32, y: i32) -> bool {
    for (sx, sy, dist) in sources {
        let dx = (sx.abs_diff(x) + sy.abs_diff(y)) as i32;
        if dx <= *dist {
            return false;
        }
    }
    true
}

fn part_one(
    sources: &HashSet<(i32, i32, i32)>,
    beacons: &HashSet<(i32, i32)>,
    search_row: i32,
) -> u32 {
    let mut ans = 0;
    for x in (-1e7 as i32)..=(1e7 as i32) {
        if !is_valid_pos(sources, x, search_row) && !beacons.contains(&(x, search_row)) {
            ans += 1;
        }
    }
    ans
}

fn part_two(sources: &HashSet<(i32, i32, i32)>, bound: i32) -> u32 {
    for (sx, sy, dist) in sources {
        for dx in 0..=(dist + 1) {
            let dy = (dist + 1) - dx;
            for (signx, signy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let x = sx + (dx * signx);
                let y = sy + (dy * signy);
                if (x >= 0 && x <= bound) && (y >= 0 && y <= bound) {
                    assert_eq!((x.abs_diff(*sx) + y.abs_diff(*sy)) as i32, *dist + 1);
                    if is_valid_pos(sources, x, y) {
                        return ((x * 4_000_000) + y) as u32;
                    }
                }
            }
        }
    }
    0
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (sources, beacons) = parse_input(&input);
    let part_one_ans = part_one(&sources, &beacons, 2_000_000);
    let part_two_ans = part_two(&sources, 4_000_000);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
