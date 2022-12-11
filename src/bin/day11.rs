use anyhow::Result;
use num::Integer;
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test: fn(u64) -> usize,
    div_by: u64,
    times_inspected: u64,
}

fn get_sample_hardcoded_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([79, 98]),
            operation: |x| x * 19,
            test: |x| if x % 23 == 0 { 2 } else { 3 },
            div_by: 23,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: |x| x + 6,
            test: |x| if x % 19 == 0 { 2 } else { 0 },
            div_by: 19,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: |x| x * x,
            test: |x| if x % 13 == 0 { 1 } else { 3 },
            div_by: 13,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([74]),
            operation: |x| x + 3,
            test: |x| if x % 17 == 0 { 0 } else { 1 },
            div_by: 17,
            times_inspected: 0,
        },
    ]
}

fn get_actual_hardcoded_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([96, 60, 68, 91, 83, 57, 85]),
            operation: |x| x * 2,
            test: |x| if x % 17 == 0 { 2 } else { 5 },
            div_by: 17,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([75, 78, 68, 81, 73, 99]),
            operation: |x| x + 3,
            test: |x| if x % 13 == 0 { 7 } else { 4 },
            div_by: 13,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([69, 86, 67, 55, 96, 69, 94, 85]),
            operation: |x| x + 6,
            test: |x| if x % 19 == 0 { 6 } else { 5 },
            div_by: 19,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([88, 75, 74, 98, 80]),
            operation: |x| x + 5,
            test: |x| if x % 7 == 0 { 7 } else { 1 },
            div_by: 7,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([82]),
            operation: |x| x + 8,
            test: |x| if x % 11 == 0 { 0 } else { 2 },
            div_by: 11,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([72, 92, 92]),
            operation: |x| x * 5,
            test: |x| if x % 3 == 0 { 6 } else { 3 },
            div_by: 3,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([74, 61]),
            operation: |x| x * x,
            test: |x| if x % 2 == 0 { 3 } else { 1 },
            div_by: 2,
            times_inspected: 0,
        },
        Monkey {
            items: VecDeque::from([76, 86, 83, 55]),
            operation: |x| x + 4,
            test: |x| if x % 5 == 0 { 4 } else { 0 },
            div_by: 5,
            times_inspected: 0,
        },
    ]
}

fn calculate_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, div_three: bool) -> u64 {
    let mod_by = monkeys.iter().fold(1, |acc, curr| acc.lcm(&curr.div_by));

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(curr_item) = monkeys[i].items.pop_front() {
                let worry_level = if div_three {
                    ((monkeys[i].operation)(curr_item) / 3) % mod_by
                } else {
                    (monkeys[i].operation)(curr_item) % mod_by
                };
                let throw_to_monkey = (monkeys[i].test)(worry_level);
                monkeys[throw_to_monkey].items.push_back(worry_level);
                monkeys[i].times_inspected += 1;
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.times_inspected);
    (&monkeys[monkeys.len() - 2..])
        .iter()
        .map(|monkey| monkey.times_inspected)
        .fold(1, |acc, curr| acc * curr)
}

fn main() -> Result<()> {
    let part_one_ans = calculate_monkey_business(get_actual_hardcoded_monkeys(), 20, true);
    let part_two_ans = calculate_monkey_business(get_actual_hardcoded_monkeys(), 10_000, false);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
