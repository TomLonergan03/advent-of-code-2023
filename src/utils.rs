use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

#[derive(PartialEq)]
pub enum NeighbourPattern {
    Plus,
    PlusWithCenter,
    All,
    AllWithCenter,
}

#[derive(PartialEq)]
pub enum NeighbourArrangement {
    Scanning,
    Clockwise,
}

pub fn get_neighbours(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    pattern: NeighbourPattern,
    arrangement: NeighbourArrangement,
) -> Vec<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    let mut result: Vec<Option<(usize, usize)>> = Vec::new();
    match pattern {
        NeighbourPattern::All => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0
                        && i < height as i32
                        && j >= 0
                        && j <= width as i32
                        && !(i == x && j == y)
                    {
                        result.push(Some((i as usize, j as usize)))
                    } else {
                        result.push(None)
                    }
                }
            }
        }
        NeighbourPattern::AllWithCenter => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0 && i < height as i32 && j >= 0 && j <= width as i32 {
                        result.push(Some((i as usize, j as usize)))
                    } else {
                        result.push(None)
                    }
                }
            }
        }
        NeighbourPattern::Plus => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0
                        && i < height as i32
                        && j >= 0
                        && j <= width as i32
                        && (i == x || j == y)
                        && !(i == x && j == y)
                    {
                        result.push(Some((i as usize, j as usize)))
                    } else {
                        result.push(None)
                    }
                }
            }
        }
        NeighbourPattern::PlusWithCenter => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0
                        && i < height as i32
                        && j >= 0
                        && j <= width as i32
                        && (i == x || j == y)
                    {
                        result.push(Some((i as usize, j as usize)))
                    } else {
                        result.push(None)
                    }
                }
            }
        }
    }
    if arrangement == NeighbourArrangement::Scanning {
        return result.into_iter().flatten().collect();
    }
    match pattern {
        NeighbourPattern::Plus => vec![result[0], result[2], result[3], result[1]]
            .into_iter()
            .flatten()
            .collect(),
        NeighbourPattern::PlusWithCenter => {
            vec![result[2], result[0], result[3], result[4], result[1]]
                .into_iter()
                .flatten()
                .collect()
        }
        NeighbourPattern::All => vec![
            result[0], result[1], result[2], result[4], result[7], result[6], result[5], result[3],
        ]
        .into_iter()
        .flatten()
        .collect(),
        NeighbourPattern::AllWithCenter => vec![
            result[4], result[0], result[1], result[2], result[5], result[8], result[7], result[6],
            result[3],
        ]
        .into_iter()
        .flatten()
        .collect(),
    }
}

pub fn count_items<A: Eq + std::hash::Hash + Clone + Copy>(
    collection: &Vec<A>,
) -> HashMap<A, usize> {
    let mut result: HashMap<A, usize> = HashMap::new();
    for item in collection {
        let count = result.entry(*item).or_insert(0);
        *count += 1;
    }
    result
}

#[derive(Debug)]
pub struct Node<A: Debug, B: Debug> {
    pub name: A,
    pub value: B,
    pub left: Option<Rc<RefCell<Node<A, B>>>>,
    pub right: Option<Rc<RefCell<Node<A, B>>>>,
}

pub fn wait_for_input() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

pub fn pretty_print_2d_array<A: Debug>(array: &[Vec<A>]) {
    for line in array {
        println!("{line:?}");
    }
}

pub fn transpose<A: Clone>(field: &[Vec<A>]) -> Vec<Vec<A>> {
    (0..field[0].len())
        .map(|i| field.iter().map(|c| c[i].clone()).collect())
        .collect::<Vec<Vec<A>>>()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
