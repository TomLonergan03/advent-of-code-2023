use std::{fmt::Debug, str::FromStr};

use crate::utils::{get_neighbours, NeighbourArrangement, NeighbourPattern};

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

#[derive(PartialEq, Clone, Copy)]
enum Location {
    GardenPlot,
    Rock,
    Visiting,
}

impl FromStr for Location {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Location::GardenPlot),
            "#" => Ok(Location::Rock),
            "S" => Ok(Location::Visiting),
            _ => Err(()),
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GardenPlot => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Visiting => write!(f, "O"),
        }
    }
}

fn _part_1(_input: &[String]) -> usize {
    let mut garden = _input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Location>().unwrap())
                .collect::<Vec<Location>>()
        })
        .collect::<Vec<Vec<Location>>>();
    for _ in 0..64 {
        for (x, y) in garden
            .clone()
            .iter()
            .enumerate()
            .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, y)| (i, j, y)))
            .filter(|(_, _, &y)| y == Location::Visiting)
            .map(|(x, y, _)| (x, y))
        {
            let neighbours = get_neighbours(
                x,
                y,
                garden.len(),
                garden[0].len(),
                NeighbourPattern::Plus,
                NeighbourArrangement::Scanning,
            );
            for (new_x, new_y) in neighbours {
                if garden[new_x][new_y] == Location::GardenPlot {
                    garden[new_x][new_y] = Location::Visiting
                }
            }
            garden[x][y] = Location::GardenPlot
        }
    }
    garden
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&&x| x == Location::Visiting)
        .count()
}

fn _part_2(_input: &[String]) -> i32 {
    -1
}
