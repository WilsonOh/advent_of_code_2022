use anyhow::Result;
use itertools::Itertools;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum TreeNode {
    Internal(Vec<TreeNode>),
    Leaf(u32),
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (TreeNode::Internal(children_one), TreeNode::Internal(children_two)) => {
                for (c1, c2) in children_one.iter().zip(children_two) {
                    if c1 != c2 {
                        return c1.partial_cmp(c2);
                    }
                }
                children_one.len().partial_cmp(&children_two.len())
            }
            (TreeNode::Internal(_), TreeNode::Leaf(val)) => {
                self.partial_cmp(&TreeNode::Internal(vec![TreeNode::Leaf(*val)]))
            }
            (TreeNode::Leaf(val), TreeNode::Internal(_)) => {
                TreeNode::Internal(vec![TreeNode::Leaf(*val)]).partial_cmp(other)
            }
            (TreeNode::Leaf(val1), TreeNode::Leaf(val2)) => val1.partial_cmp(val2),
        }
    }
}

fn parse_node(chars: &mut Chars) -> TreeNode {
    let mut st: Vec<TreeNode> = vec![];
    // for handling multi-digit numbers
    let mut tmp = String::new();
    while let Some(curr) = (*chars).next() {
        match curr {
            '[' => st.push(parse_node(chars)),
            ']' => {
                if let Ok(val) = tmp.parse::<u32>() {
                    st.push(TreeNode::Leaf(val));
                }
                return TreeNode::Internal(st);
            }
            ',' => {
                if let Ok(val) = tmp.parse::<u32>() {
                    st.push(TreeNode::Leaf(val));
                    tmp.clear();
                }
            }
            val => tmp.push(val),
        }
    }
    TreeNode::Internal(st)
}

fn part_one(input: &str) -> u32 {
    let node_pairs = input
        .trim()
        .split("\n\n")
        .filter_map(|line| {
            line.split("\n")
                .map(|node| parse_node(&mut node.chars()))
                .collect_tuple()
        })
        .collect_vec();
    node_pairs.iter().zip(1..).fold(
        0,
        |acc, ((left, right), idx)| {
            if left < right {
                acc + idx
            } else {
                acc
            }
        },
    )
}

fn part_two(input: &str) -> u32 {
    let mut nodes = input
        .trim()
        .split("\n\n")
        .flat_map(|pair| pair.split("\n"))
        .map(|node| parse_node(&mut node.chars()))
        .collect_vec();
    for node in &nodes {
        println!("{node:?}");
    }
    let dividers = vec![
        parse_node(&mut "[[2]]".chars()),
        parse_node(&mut "[[6]]".chars()),
    ];
    nodes.append(&mut dividers.clone());
    nodes.sort();
    let mut ans = 1;
    for (node, idx) in nodes.iter().zip(1..) {
        if *node == dividers[0] || *node == dividers[1] {
            ans *= idx;
        }
    }
    ans
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("sample.txt")?;
    let part_one_ans = part_one(&input);
    let part_two_ans = part_two(&input);
    println!("part one: {part_one_ans}");
    println!("part two: {part_two_ans}");
    Ok(())
}
