use std::{collections::VecDeque, str::FromStr};

use crate::utils::{get_neighbours, NeighbourArrangement, NeighbourPattern};

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn from_coords(
        old_x: usize,
        old_y: usize,
        new_x: usize,
        new_y: usize,
    ) -> Option<Direction> {
        let (old_x, old_y, new_x, new_y) = (old_x as i32, old_y as i32, new_x as i32, new_y as i32);
        if old_x == new_x {
            if new_y - old_y == 1 {
                Some(Direction::North)
            } else if new_y - old_y == -1 {
                Some(Direction::South)
            } else {
                None
            }
        } else if old_y == new_y {
            if new_x - old_x == 1 {
                Some(Direction::East)
            } else if new_x - old_x == -1 {
                Some(Direction::West)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn reverse(&self) -> Self {
        match *self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NECorner,
    NWCorner,
    SWCorner,
    SECorner,
    Ground,
    Start,
    Visited(u32),
}

impl FromStr for Pipe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Pipe::Vertical),
            "-" => Ok(Pipe::Horizontal),
            "L" => Ok(Pipe::NECorner),
            "J" => Ok(Pipe::NWCorner),
            "7" => Ok(Pipe::SWCorner),
            "F" => Ok(Pipe::SECorner),
            "." => Ok(Pipe::Ground),
            "S" => Ok(Pipe::Start),
            _ => Err("Invalid pipe".to_string()),
        }
    }
}

impl Pipe {
    pub fn is_connected(&self, direction: Direction) -> bool {
        // match direction {
        //     Direction::North
        // }
        todo!()
    }

    fn direction(&self) -> (Direction, Direction) {
        match self {
            Pipe::Vertical => (Direction::North, Direction::South),
            Pipe::Horizontal => (Direction::East, Direction::West),
            Pipe::NECorner => (Direction::North, Direction::East),
            Pipe::NWCorner => (Direction::North, Direction::West),
            Pipe::SWCorner => (Direction::South, Direction::West),
            Pipe::SECorner => (Direction::South, Direction::East),
            x => panic!("Invalid pipe {x:?}"),
        }
    }
}

fn add_connected_pipes(
    pipes: &[Vec<Pipe>],
    to_visit: &mut VecDeque<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) {
    let next_steps = get_neighbours(
        start_x,
        start_y,
        pipes.len(),
        pipes[0].len(),
        NeighbourPattern::Plus,
        NeighbourArrangement::Scanning,
    );
    for (x, y) in next_steps {
        let direction = Direction::from_coords(start_x, start_y, x, y);
        if direction.is_none() {
            continue;
        }
        if pipes[x][y].is_connected(direction.unwrap()) {
            to_visit.push_back((x, y))
        };
    }
}

fn _part_1(_input: &[String]) -> u32 {
    let mut pipes: Vec<Vec<Pipe>> = _input
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse::<Pipe>().unwrap())
                .collect()
        })
        .collect();
    let (start_x, start_y, _) = pipes
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.iter().enumerate()))
        .flat_map(|(i, xs)| xs.map(|(j, y)| (i, j, y)).collect::<Vec<_>>())
        .find(|(_, _, &pipe)| pipe == Pipe::Start)
        .unwrap();
    let mut steps = 0;
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    pipes[start_x][start_y] = Pipe::Visited(0);
    let first_steps = get_neighbours(
        start_x,
        start_y,
        pipes.len(),
        pipes[0].len(),
        NeighbourPattern::Plus,
        NeighbourArrangement::Scanning,
    );
    for (x, y) in first_steps {
        let direction = Direction::from_coords(start_x, start_y, x, y);
        if direction.is_none() {
            continue;
        }
        if pipes[x][y].is_connected(direction.unwrap().reverse()) {
            to_visit.push_back((x, y))
        };
    }
    while !to_visit.is_empty() {
        for _ in 0..2 {
            let (x, y) = to_visit.pop_front().unwrap();
            pipes[x][y] = Pipe::Visited(steps);
            add_connected_pipes(&pipes, &mut to_visit, x, y);
        }
        steps += 1
    }
    for line in &pipes {
        println!("{line:?}");
    }
    *pipes
        .iter()
        .flat_map(|x| {
            x.iter().filter_map(|y| match y {
                Pipe::Visited(steps) => Some(steps),
                _ => None,
            })
        })
        .max()
        .unwrap()
}

fn _part_2(_input: &[String]) -> i32 {
    -1
}
