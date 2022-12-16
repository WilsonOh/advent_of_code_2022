use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

fn parse_graph(input: &str) -> (HashMap<String, Vec<String>>, HashMap<String, u32>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_rates: HashMap<String, u32> = HashMap::new();
    let re = Regex::new(
        r"Valve (?P<node>[[:upper:]]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<neighbours>[, A-Z]*)",
    )
    .unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let node = &captures["node"];
        let flow_rate: u32 = (&captures["flow_rate"]).parse().unwrap();
        let neighbours = (&captures["neighbours"])
            .split(", ")
            .map(|s| s.to_string())
            .collect_vec();
        flow_rates.insert(node.to_string(), flow_rate);
        graph.insert(node.to_string(), neighbours);
    }
    (graph, flow_rates)
}

fn hash_strings(opened: &HashSet<String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    opened.iter().fold(0, |acc, curr| {
        curr.hash(&mut hasher);
        acc ^ hasher.finish()
    })
}

fn dfs(
    adj_list: &HashMap<String, Vec<String>>,
    opened: &mut HashSet<String>,
    curr_node: String,
    flow_rates: &HashMap<String, u32>,
    time_left: i32,
    dp: &mut HashMap<(String, u64, i32, u32), i32>,
    elephants: u32,
) -> i32 {
    if time_left == 0 {
        return if elephants > 0 {
            dfs(
                adj_list,
                opened,
                "AA".to_string(),
                flow_rates,
                26,
                dp,
                elephants - 1,
            )
        } else {
            0
        };
    }
    let state = (
        curr_node.clone(),
        hash_strings(opened),
        time_left,
        elephants,
    );
    if dp.contains_key(&state) {
        return dp[&state];
    }
    let mut best = 0;
    if !opened.contains(&curr_node) && flow_rates[&curr_node] > 0 {
        let curr_pressure = (time_left - 1) * (flow_rates[&curr_node] as i32);
        opened.insert(curr_node.clone());
        best = best.max(
            curr_pressure
                + dfs(
                    adj_list,
                    opened,
                    curr_node.clone(),
                    flow_rates,
                    time_left - 1,
                    dp,
                    elephants,
                ),
        );
        opened.remove(&curr_node);
    }
    for nei in &adj_list[&curr_node] {
        best = best.max(dfs(
            adj_list,
            opened,
            nei.to_string(),
            flow_rates,
            time_left - 1,
            dp,
            elephants,
        ));
    }
    dp.insert(state, best);
    best
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (adj_list, flow_rates) = parse_graph(&input);
    let mut dp = HashMap::new();
    let mut opened = HashSet::new();
    let part_one = dfs(
        &adj_list,
        &mut opened,
        "AA".to_string(),
        &flow_rates,
        30,
        &mut dp,
        0,
    );
    /* let part_two = dfs(
        &adj_list,
        &mut opened,
        "AA".to_string(),
        &flow_rates,
        26,
        &mut dp,
        1,
    ); */
    println!("part one: {part_one}");
    Ok(())
}
