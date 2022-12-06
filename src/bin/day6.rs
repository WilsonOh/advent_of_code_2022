use anyhow::Result;
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
        .as_str()
        .as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| {
            let s: HashSet<&u8> = HashSet::from_iter(window.iter());
            s.len() == n
        })
        .unwrap()
        .0
        + n
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = find_idx_after_n_unique_chars(&input, 4);
    let part_two_ans = find_idx_after_n_unique_chars_alt(&input, 14);
    println!("Part one: {part_one_ans}");
    println!("Part two: {part_two_ans}");
    Ok(())
}
