use std::{collections::HashSet, str::FromStr};

use crate::utils::{
    get_neighbours, pretty_print_2d_array, Direction, NeighbourArrangement, NeighbourPattern,
};

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

#[derive(PartialEq, Debug)]
enum Tile {
    Path { slope: Direction },
    Forest,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Path {
                slope: Direction::None,
            }),
            ">" => Ok(Tile::Path {
                slope: Direction::Right,
            }),
            "<" => Ok(Tile::Path {
                slope: Direction::Left,
            }),
            "^" => Ok(Tile::Path {
                slope: Direction::Up,
            }),
            "v" => Ok(Tile::Path {
                slope: Direction::Down,
            }),
            "#" => Ok(Tile::Forest),
            _ => Err(format!("Invalid tile: {}", s)),
        }
    }
}

fn _part_1(_input: &[String]) -> usize {
    let forest = _input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Tile>().unwrap())
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: Vec<(usize, usize, usize)> = vec![(1, 0, 0)];
    let mut distance_to = vec![vec![0; forest[0].len()]; forest.len()];
    while let Some((x, y, distance)) = to_visit.pop() {
        println!("Visiting ({}, {})", x, y);
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        println!("found tile: {:?}", forest[y][x]);
        match forest[y][x] {
            Tile::Path { slope } => match slope {
                Direction::None => {
                    let neighbors = get_neighbours(
                        x,
                        y,
                        forest.len(),
                        forest[0].len(),
                        NeighbourPattern::Plus,
                        NeighbourArrangement::Clockwise,
                    );
                    println!("neighbors: {:?}", neighbors);
                    for neighbour in neighbors {
                        if forest[neighbour.1][neighbour.0] != Tile::Forest
                            && distance_to[neighbour.1][neighbour.0] > distance + 1
                        {
                            distance_to[neighbour.1][neighbour.0] = distance + 1;
                            to_visit.push((neighbour.0, neighbour.1, distance + 1));
                        }
                    }
                }
                Direction::Up => {
                    if forest[y - 1][x] != Tile::Forest
                        && y > 0
                        && distance_to[y - 1][x] > distance + 1
                    {
                        distance_to[y - 1][x] = distance + 1;
                        to_visit.push((x, y - 1, distance + 1));
                    }
                }
                Direction::Down => {
                    if forest[y + 1][x] != Tile::Forest
                        && y < forest.len() - 1
                        && distance_to[y + 1][x] > distance + 1
                    {
                        distance_to[y + 1][x] = distance + 1;
                        to_visit.push((x, y + 1, distance + 1));
                    }
                }
                Direction::Left => {
                    if forest[y][x - 1] != Tile::Forest
                        && x > 0
                        && distance_to[y][x - 1] > distance + 1
                    {
                        distance_to[y][x - 1] = distance + 1;
                        to_visit.push((x - 1, y, distance + 1));
                    }
                }
                Direction::Right => {
                    if forest[y][x + 1] != Tile::Forest
                        && x < forest[0].len() - 1
                        && distance_to[y][x + 1] > distance + 1
                    {
                        distance_to[y][x + 1] = distance + 1;
                        to_visit.push((x + 1, y, distance + 1));
                    }
                }
            },
            Tile::Forest => {
                panic!("Found forest at ({}, {})", x, y);
            }
        }
    }
    pretty_print_2d_array(&distance_to);
    *distance_to.last().unwrap().iter().max().unwrap()
}

fn _part_2(_input: &[String]) -> i32 {
    -1
}
