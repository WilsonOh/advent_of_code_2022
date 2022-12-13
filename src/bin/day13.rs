use std::str::Chars;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TreeNode {
    Internal(Vec<TreeNode>),
    Leaf(u32),
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (TreeNode::Internal(children_one), TreeNode::Internal(children_two)) => {
                for (c1, c2) in children_one.iter().zip(children_two) {
                    let res = c1.partial_cmp(c2);
                    if res != Some(std::cmp::Ordering::Equal) {
                        return res;
                    }
                }
                children_one.len().partial_cmp(&children_two.len())
            }
            (TreeNode::Internal(children), TreeNode::Leaf(_)) => {
                if children.is_empty() {
                    return Some(std::cmp::Ordering::Less);
                }
                children[0].partial_cmp(other)
            }
            (TreeNode::Leaf(_), TreeNode::Internal(children)) => {
                if children.is_empty() {
                    return Some(std::cmp::Ordering::Greater);
                }
                self.partial_cmp(&children[0])
            }
            (TreeNode::Leaf(val1), TreeNode::Leaf(val2)) => val1.partial_cmp(val2),
        }
    }
}

fn parse_node(chars: &mut Chars) -> TreeNode {
    let mut st: Vec<TreeNode> = vec![];
    let mut tmp = String::new();
    while let Some(curr) = (*chars).next() {
        match curr {
            '[' => st.push(parse_node(chars)),
            ',' | ']' => {
                if !tmp.is_empty() {
                    st.push(TreeNode::Leaf(tmp.parse::<u32>().unwrap()));
                    tmp.clear()
                }
            }
            val => tmp.push(val),
        }
    }
    TreeNode::Internal(st)
}

fn parse_input(input: &str) -> Vec<(TreeNode, TreeNode)> {
    input
        .split("\n\n")
        .filter_map(|line| {
            line.split("\n")
                .map(|node| parse_node(&mut node.chars()))
                .collect_tuple()
        })
        .collect_vec()
}

fn part_one(nodes: &[(TreeNode, TreeNode)]) -> u32 {
    nodes.iter().zip(1..).fold(
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

fn main() -> Result<()> {
    let input = std::fs::read_to_string("sample.txt")?;
    let nodes = parse_input(&input);
    let part_one_ans = part_one(&nodes);
    println!("part one: {part_one_ans}");
    Ok(())
}
