use std::collections::VecDeque;

use anyhow::Result;

fn parse_list(input: &str) -> VecDeque<i32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn part_one(input: &str) -> u32 {
    let mut nums = parse_list(input);
    let mut indices: VecDeque<usize> = (0..nums.len()).collect();
    let len = nums.len() as i32;
    for i in 0..indices.len() {
        let idx = indices[i];
        let curr = nums[idx] + idx as i32;
        let mut oor = true;
        let new_idx = if curr >= 0 && curr < len {
            oor = false;
            curr
        } else if curr < 0 {
            (curr - 1).rem_euclid(len)
        } else {
            (curr + 1).rem_euclid(len)
        };
        println!(
            "num = {curr} idx = {idx} new_idx = {new_idx} nums: {nums:?} indices: {indices:?}"
        );
        assert!(
            new_idx >= 0 && new_idx < len,
            "new index is within valid bounds"
        );
        let removed = nums.remove(idx).unwrap();
        nums.insert(new_idx as usize, removed);
        let removed_idx = indices.remove(idx).unwrap();
        if oor {
            indices.insert(new_idx as usize - 1, removed_idx);
        } else {
            indices.insert(new_idx as usize, removed_idx);
        }
    }
    0
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("sample.txt")?;
    part_one(&input);
    Ok(())
}
