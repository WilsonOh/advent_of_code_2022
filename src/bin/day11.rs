use anyhow::Result;
use itertools::Itertools;
use num::Integer;
use std::collections::VecDeque;

struct Monkey<'a> {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64 + 'a>,
    test_func: Box<dyn Fn(u64) -> usize + 'a>,
    test_num: u64,
    times_inspected: u64,
}

fn parse_monkey(monkey: &str) -> Monkey {
    let lines = monkey.lines().collect_vec();
    let items: VecDeque<u64> = lines[1]
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();
    let (op, val) = lines[2]
        .strip_prefix("  Operation: new = old ")
        .unwrap()
        .split_whitespace()
        .collect_tuple()
        .unwrap();
    let operation: Box<dyn Fn(u64) -> u64> = match (op, val) {
        ("*", "old") => Box::new(|x| x * x),
        ("*", val) => Box::new(move |x| x * val.parse::<u64>().unwrap()),
        ("+", val) => Box::new(move |x| x + val.parse::<u64>().unwrap()),
        _ => panic!("invalid operation"),
    };
    let test_num: u64 = lines[3]
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    let true_monkey: usize = lines[4]
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    let false_monkey: usize = lines[5]
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    let test_func: Box<dyn Fn(u64) -> usize> = Box::new(move |x| {
        if x % test_num == 0 {
            true_monkey
        } else {
            false_monkey
        }
    });
    Monkey {
        items,
        operation,
        test_func,
        test_num,
        times_inspected: 0,
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect_vec()
}

fn calculate_monkey_business(input: &str, rounds: usize, div_by: u64) -> u64 {
    let mut monkeys = get_monkeys(input);
    let mod_by = monkeys.iter().fold(1, |acc, curr| acc.lcm(&curr.test_num));
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(curr_item) = monkeys[i].items.pop_front() {
                let worry_level = ((monkeys[i].operation)(curr_item) / div_by) % mod_by;
                let throw_to_monkey = (monkeys[i].test_func)(worry_level);
                monkeys[throw_to_monkey].items.push_back(worry_level);
                monkeys[i].times_inspected += 1;
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.times_inspected);
    monkeys[monkeys.len() - 2..]
        .iter()
        .map(|monkey| monkey.times_inspected)
        .product()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = calculate_monkey_business(&input, 20, 3);
    let part_two_ans = calculate_monkey_business(&input, 10_000, 1);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
