use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr, vec};

use crate::utils::Node;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

enum Move {
    Left,
    Right,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Move::Right),
            "L" => Ok(Move::Left),
            _ => Err("Invalid move".to_string()),
        }
    }
}

impl FromStr for Node<String, Vec<String>> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node {
            name: s.split_whitespace().next().unwrap().to_string(),
            value: s
                .split_whitespace()
                .skip(2)
                .take(2)
                .map(|x| {
                    x.chars()
                        .filter(|y| y.is_alphanumeric())
                        .map(|z| z.to_string())
                        .collect()
                })
                .collect(),
            left: None,
            right: None,
        })
    }
}

type RefNode = Rc<RefCell<Node<String, Vec<String>>>>;

type UnconnectedGraph = HashMap<String, RefNode>;

type RepeatingPath<'a> = std::iter::Cycle<std::slice::Iter<'a, Move>>;

fn connect_nodes(unconnected_nodes: &UnconnectedGraph, start_node: RefNode) -> RefNode {
    for (_, node) in unconnected_nodes.iter() {
        let mut node = node.borrow_mut();
        node.left = Some(unconnected_nodes[&node.value[0]].clone());
        node.right = Some(unconnected_nodes[&node.value[1]].clone())
    }
    start_node
}

fn make_move(current_node: RefNode, next_move: &Move) -> RefNode {
    match next_move {
        Move::Left => {
            return current_node.borrow().left.clone().unwrap().clone();
        }
        Move::Right => {
            return current_node.borrow().right.clone().unwrap().clone();
        }
    }
}

fn navigate(start: RefNode, path: RepeatingPath) -> usize {
    let mut current_node: RefNode = start;
    for (move_count, next_move) in path.enumerate() {
        if &current_node.borrow().name == "ZZZ" {
            return move_count;
        }
        current_node = make_move(current_node, next_move);
    }
    unreachable!("No route to end found")
}

fn _part_1(_input: &[String]) -> usize {
    let moves = _input[0]
        .chars()
        .map(|x| x.to_string().parse::<Move>().unwrap())
        .collect::<Vec<_>>();
    let path: RepeatingPath = moves.iter().cycle();
    let unconnected_nodes: UnconnectedGraph = _input[2..]
        .iter()
        .map(|x| {
            Rc::new(RefCell::new(
                x.parse::<Node<String, Vec<String>>>().unwrap(),
            ))
        })
        .map(|x: RefNode| (x.borrow().name.clone(), x.clone()))
        .collect();
    let start_node = unconnected_nodes["AAA"].clone();
    let start: RefNode = connect_nodes(&unconnected_nodes, start_node);
    navigate(start, path)
}

fn navigate_all(nodes: Vec<RefNode>, path: RepeatingPath) -> usize {
    let mut cycle_length = vec![0; nodes.len()];
    let mut nodes = nodes;
    for (move_count, next_move) in path.enumerate() {
        nodes.iter().enumerate().for_each(|(i, x)| {
            if x.borrow().name.ends_with('Z') && cycle_length[i] == 0 {
                cycle_length[i] = move_count;
            }
        });
        if cycle_length.iter().all(|&x| x != 0) {
            break;
        }
        nodes = nodes
            .iter()
            .enumerate()
            .map(|(i, x)| {
                if cycle_length[i] == 0 {
                    make_move(x.clone(), next_move)
                } else {
                    x.clone()
                }
            })
            .collect();
    }
    cycle_length
        .iter()
        .fold(1, |acc, &x| num::integer::lcm(acc, x))
}

fn _part_2(_input: &[String]) -> usize {
    let moves = _input[0]
        .chars()
        .map(|x| x.to_string().parse::<Move>().unwrap())
        .collect::<Vec<_>>();
    let path: RepeatingPath = moves.iter().cycle();
    let unconnected_nodes: UnconnectedGraph = _input[2..]
        .iter()
        .map(|x| {
            Rc::new(RefCell::new(
                x.parse::<Node<String, Vec<String>>>().unwrap(),
            ))
        })
        .map(|x: RefNode| (x.borrow().name.clone(), x.clone()))
        .collect();
    let _ = connect_nodes(
        &unconnected_nodes,
        unconnected_nodes.iter().next().unwrap().1.clone(),
    );
    let current_nodes: Vec<RefNode> = unconnected_nodes
        .iter()
        .filter(|(name, _)| name.ends_with('A'))
        .map(|x| x.1.clone())
        .collect();
    navigate_all(current_nodes, path)
}
