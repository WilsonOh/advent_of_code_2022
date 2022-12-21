use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
enum ExprTreeNode<'a> {
    Operator {
        op: fn(i128, i128) -> i128,
        left: &'a str,
        right: &'a str,
    },
    Operand {
        val: i128,
    },
}

type AdjList<'a> = HashMap<&'a str, ExprTreeNode<'a>>;

fn parse_tree(input: &str) -> AdjList {
    let mut adj_list: HashMap<&str, ExprTreeNode> = HashMap::new();
    for line in input.lines() {
        let (node, rest) = line.trim().split(": ").collect_tuple().unwrap();
        let tokens = rest.split(' ').collect_vec();
        if tokens.len() == 1 {
            adj_list.insert(
                node,
                ExprTreeNode::Operand {
                    val: tokens[0].parse().unwrap(),
                },
            );
        } else {
            let (op, left, right) = (tokens[1], tokens[0], tokens[2]);
            let op_func = match op {
                "+" => |a: i128, b: i128| a + b,
                "-" => |a: i128, b: i128| a - b,
                "*" => |a: i128, b: i128| a * b,
                "/" => |a: i128, b: i128| a / b,
                _ => panic!("invalid operator"),
            };
            adj_list.insert(
                node,
                ExprTreeNode::Operator {
                    op: op_func,
                    left,
                    right,
                },
            );
        }
    }
    adj_list
}

fn eval_expr(adj_list: &AdjList, curr_node: &str) -> i128 {
    match adj_list[curr_node] {
        ExprTreeNode::Operator { op, left, right } => {
            op(eval_expr(adj_list, left), eval_expr(adj_list, right))
        }
        ExprTreeNode::Operand { val } => val,
    }
}

fn get_root_operands(adj_list: &mut AdjList, humn_num: i128) -> (i128, i128) {
    adj_list.insert("humn", ExprTreeNode::Operand { val: humn_num });
    if let ExprTreeNode::Operator { op: _, left, right } = adj_list["root"] {
        return (eval_expr(adj_list, left), eval_expr(adj_list, right));
    }
    unreachable!()
}

fn part_one(input: &str) -> i128 {
    let adj_list = parse_tree(input);
    eval_expr(&adj_list, "root")
}

fn part_two(input: &str) -> i128 {
    let mut adj_list = parse_tree(input);
    if let ExprTreeNode::Operator { op: _, left, right } = adj_list["root"] {
        let new_root_op = |a: i128, b: i128| a - b;
        adj_list.insert(
            "root",
            ExprTreeNode::Operator {
                op: new_root_op,
                left,
                right,
            },
        );
    }
    let mut low: i128 = 0;
    let mut high: i128 = i64::MAX as i128;
    let orig_ord = match eval_expr(&adj_list, "root") {
        val if val < 0 => std::cmp::Ordering::Less,
        val if val > 0 => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    };
    while low < high {
        let mid = low + (high - low) / 2;
        let (left, right) = get_root_operands(&mut adj_list, mid);
        match left.cmp(&right) {
            std::cmp::Ordering::Equal => high = mid,
            cmp => {
                if cmp == orig_ord {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
    }
    low
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_one_ans = part_one(&input);
    let part_two_ans = part_two(&input);
    println!("part one : {part_one_ans}");
    println!("part two : {part_two_ans}");
    Ok(())
}
