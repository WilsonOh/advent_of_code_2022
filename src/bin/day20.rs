use anyhow::Result;
use std::collections::VecDeque;

fn mix(input: &str, encryption_key: i64, num_mixes: usize) -> i64 {
    let mut nums: VecDeque<(usize, i64)> = input
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .map(|num| num * encryption_key)
        .enumerate()
        .collect();
    for _ in 0..num_mixes {
        for i in 0..nums.len() {
            while nums[0].0 != i {
                nums.rotate_left(1);
            }
            let val = nums.pop_front().unwrap();
            let shift_num = val.1.rem_euclid(nums.len() as i64);
            nums.rotate_left(shift_num as usize);
            nums.push_back(val);
        }
    }
    let zero_idx = nums.iter().position(|(_, val)| *val == 0).unwrap();
    (1..=3)
        .map(|i| nums[(zero_idx + (i * 1000)).rem_euclid(nums.len())].1 as i64)
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = mix(&input, 1, 1);
    let part_two_ans = mix(&input, 811589153, 10);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
