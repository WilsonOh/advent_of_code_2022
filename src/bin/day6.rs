use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn find_idx_after_n_unique_chars(input: &str, n: usize) -> usize {
    let mut dq: VecDeque<char> = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();
    for (idx, c) in input.chars().enumerate() {
        if set.contains(&c) {
            while let Some(val) = dq.pop_front() {
                set.remove(&val);
                if val == c {
                    break;
                }
            }
        }
        dq.push_back(c);
        set.insert(c);
        if dq.len() > n {
            if let Some(val) = dq.pop_front() {
                set.remove(&val);
            }
        }
        if set.len() == n && dq.len() == n {
            return idx + 1;
        }
    }
    unreachable!()
}

fn find_idx_after_n_unique_chars_alt(input: &str, n: usize) -> usize {
    input
        .chars()
        .collect_vec() // need to collect into a vector as chars do not have a windows method
        .windows(n)
        .enumerate()
        .find(|(_, window)| window.iter().all_unique()) // find the first unique window of n
                                                        // elements
        .unwrap()
        .0 // get index
        + n // add n to get the first index after n unique chars
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = find_idx_after_n_unique_chars(&input, 4);
    let part_two_ans = find_idx_after_n_unique_chars_alt(&input, 14);
    println!("Part one: {part_one_ans}");
    println!("Part two: {part_two_ans}");
    Ok(())
}
