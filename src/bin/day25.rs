use std::collections::HashMap;

use anyhow::Result;
use num::Integer;

fn parse_snafu(snafu: &str) -> i64 {
    let snafu_map = HashMap::from([('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2)]);
    let mut ans = 0i64;
    for (e, n) in snafu.chars().enumerate() {
        let e = (snafu.len() - e - 1) as u32;
        let n = snafu_map[&n];
        ans += n * 5i64.pow(e);
    }
    ans
}

fn part_one(input: &str) -> String {
    let mut ans = String::new();
    let mut c: i64 = input.lines().map(parse_snafu).sum();
    while c > 0 {
        let (n, r) = c.div_rem(&5);
        println!("n = {n} r = {r}");
        c = n;
        match r {
            r if r < 3 => ans.push(char::from_digit(r as u32, 10).unwrap()),
            3 => {
                ans.push('=');
                c += 1;
            }
            4 => {
                ans.push('-');
                c += 1;
            }
            _ => panic!("invalid character"),
        }
    }
    ans.chars().rev().collect()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = part_one(&input);
    println!("part one: {part_one_ans}");
    Ok(())
}
