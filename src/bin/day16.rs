use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_graph(input: &str) -> (HashMap<String, Vec<String>>, HashMap<String, u32>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_rates: HashMap<String, u32> = HashMap::new();
    let re = Regex::new(
        r"Valve (?P<node>[[:upper:]]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<neighbours>.*)",
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

fn dfs(
    adj_list: &HashMap<String, Vec<String>>,
    opened: &mut HashSet<String>,
    curr_node: String,
    flow_rates: &HashMap<String, u32>,
    time_left: u32,
) -> u32 {
    if time_left <= 0 {
        return 0;
    }
    let mut best = 0;
    if !opened.contains(&curr_node) {
        let curr_pressure = (time_left - 1) * flow_rates[&curr_node];
        for nei in &adj_list[&curr_node] {
            if curr_pressure != 0 {
                opened.insert(curr_node.clone());
            }
            best = best.max(
                curr_pressure
                    + dfs(
                        adj_list,
                        opened,
                        nei.to_string(),
                        flow_rates,
                        if curr_pressure == 0 {
                            time_left - 1
                        } else {
                            time_left - 2
                        },
                    ),
            );
            opened.remove(&curr_node);
        }
    } else {
        for nei in &adj_list[&curr_node] {
            best = best.max(dfs(
                adj_list,
                opened,
                nei.to_string(),
                flow_rates,
                time_left - 1,
            ));
        }
    }
    best
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("sample.txt")?;
    let (adj_list, flow_rates) = parse_graph(&input);
    let mut opened = HashSet::new();
    let ans = dfs(&adj_list, &mut opened, "AA".to_string(), &flow_rates, 30);
    println!("part one: {ans}");
    Ok(())
}
