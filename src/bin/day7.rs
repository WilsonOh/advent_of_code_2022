use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use itertools::Itertools;

const TOTAL_SPACE: u32 = 70000000;
const SPACE_NEEDED: u32 = 30000000;

fn get_file_tree_sizes(input: &str) -> HashMap<String, u32> {
    let mut lines = input.lines().peekable();
    let mut curr_path: Vec<&str> = vec![];
    let mut size_map: HashMap<String, u32> = HashMap::new();
    while let Some(next) = lines.next() {
        let tokens = next.split_whitespace().collect_vec();
        match tokens[1] {
            "cd" => {
                if tokens[2] == ".." {
                    curr_path.pop();
                } else {
                    curr_path.push(tokens[2])
                }
            }
            "ls" => {
                let total_size = lines
                    // take until we hit the next command
                    .peeking_take_while(|line| !line.starts_with('$'))
                    .fold(0u32, |acc, file| {
                        let (file_type, _) = file.split_whitespace().collect_tuple().unwrap();
                        if let Ok(size) = file_type.parse::<u32>() {
                            acc + size
                        } else {
                            acc
                        }
                    });
                // Update filesize for every single ancestor directory
                for i in 0..curr_path.len() {
                    let path = curr_path[0..=i].join("/");
                    size_map
                        .entry(path)
                        .and_modify(|size| *size += total_size)
                        .or_insert(total_size);
                }
            }
            _ => panic!("invalid command"),
        }
    }
    size_map
}

fn part_one(size_map: &HashMap<String, u32>) -> u32 {
    size_map
        .iter()
        .map(|(_, v)| v)
        .filter(|size| **size <= 100000)
        .sum()
}

fn part_two(size_map: &HashMap<String, u32>) -> u32 {
    let unused_space = TOTAL_SPACE - size_map.get("/").unwrap();
    let need_to_delete = SPACE_NEEDED - unused_space;
    let sizes: BTreeSet<u32> = size_map.iter().map(|(_, v)| *v).collect();
    // use bBST "lower_bound" to find the first size >= need_to_delete
    let ans = sizes.range(need_to_delete..).next().cloned().unwrap();
    ans
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let size_map = get_file_tree_sizes(&input);
    let ans = part_two(&size_map);
    println!("{ans}");
    Ok(())
}
