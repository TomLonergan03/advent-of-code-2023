use std::collections::{HashSet, VecDeque};

use crate::utils::{pretty_print_2d_array, Direction};

use rayon::prelude::*;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Beam {
    pub location: (usize, usize),
    pub direction: Direction,
}

enum ReflectResult {
    Reflect(Beam),
    Split(Beam, Beam),
    Absorb,
}

fn reflect(board: &[Vec<char>], x: usize, y: usize, direction: Direction) -> ReflectResult {
    match direction {
        Direction::Up => match board[y][x] {
            '\\' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Left),
            }),
            '/' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Right),
            }),
            '-' => ReflectResult::Split(
                Beam {
                    location: (x, y),
                    direction: Direction::Left,
                },
                Beam {
                    location: (x, y),
                    direction: Direction::Right,
                },
            ),
            _ => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: Direction::Up,
            }),
        },
        Direction::Down => match board[y][x] {
            '\\' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Right),
            }),
            '/' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Left),
            }),
            '-' => ReflectResult::Split(
                Beam {
                    location: (x, y),
                    direction: Direction::Left,
                },
                Beam {
                    location: (x, y),
                    direction: Direction::Right,
                },
            ),
            _ => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: Direction::Down,
            }),
        },
        Direction::Left => match board[y][x] {
            '\\' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Up),
            }),
            '/' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Down),
            }),
            '|' => ReflectResult::Split(
                Beam {
                    location: (x, y),
                    direction: Direction::Up,
                },
                Beam {
                    location: (x, y),
                    direction: Direction::Down,
                },
            ),
            _ => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: Direction::Left,
            }),
        },
        Direction::Right => match board[y][x] {
            '\\' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Down),
            }),
            '/' => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: (Direction::Up),
            }),
            '|' => ReflectResult::Split(
                Beam {
                    location: (x, y),
                    direction: Direction::Up,
                },
                Beam {
                    location: (x, y),
                    direction: Direction::Down,
                },
            ),
            _ => ReflectResult::Reflect(Beam {
                location: (x, y),
                direction: Direction::Right,
            }),
        },
        _ => unreachable!(),
    }
}

fn next_position(board: &[Vec<char>], x: usize, y: usize, direction: Direction) -> ReflectResult {
    match direction {
        Direction::Up => {
            if y > 0 {
                let y = y - 1;
                reflect(board, x, y, direction)
            } else {
                ReflectResult::Absorb
            }
        }
        Direction::Down => {
            if y < board[0].len() - 1 {
                let y = y + 1;
                reflect(board, x, y, direction)
            } else {
                ReflectResult::Absorb
            }
        }
        Direction::Left => {
            if x > 0 {
                let x = x - 1;
                reflect(board, x, y, direction)
            } else {
                ReflectResult::Absorb
            }
        }
        Direction::Right => {
            if x < board.len() - 1 {
                let x = x + 1;
                reflect(board, x, y, direction)
            } else {
                ReflectResult::Absorb
            }
        }
        _ => unreachable!(),
    }
}

fn _debug_illumination(board: &[Vec<char>], energised: &[Vec<u32>]) {
    let pretty_board: Vec<Vec<_>> = board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| {
                    if tile != &'.' {
                        tile
                    } else if energised[y][x] == 1 {
                        &'#'
                    } else {
                        &'.'
                    }
                })
                .collect()
        })
        .collect();
    pretty_print_2d_array(&pretty_board);
}

fn calculate_energy(
    board: &[Vec<char>],
    start_location: (usize, usize),
    start_direction: Direction,
) -> u32 {
    let mut energised = vec![vec![0; board[0].len()]; board.len()];
    let mut beams: VecDeque<Beam> = VecDeque::new();
    match reflect(board, start_location.0, start_location.1, start_direction) {
        ReflectResult::Reflect(beam) => beams.push_back(beam),
        ReflectResult::Split(beam1, beam2) => {
            beams.push_back(beam1);
            beams.push_back(beam2)
        }
        ReflectResult::Absorb => {}
    }
    let mut beams_have_been: HashSet<Beam> = HashSet::new();
    while !beams.is_empty() {
        let current_beam = beams.pop_front().unwrap();
        if beams_have_been.contains(&current_beam) {
            continue;
        }
        beams_have_been.insert(current_beam);
        let (x, y) = current_beam.location;
        energised[y][x] = 1;
        match next_position(board, x, y, current_beam.direction) {
            ReflectResult::Reflect(beam) => beams.push_back(beam),
            ReflectResult::Split(beam1, beam2) => {
                beams.push_back(beam1);
                beams.push_back(beam2)
            }
            ReflectResult::Absorb => {}
        }
    }
    energised.iter().flat_map(|x| x.iter()).sum()
}

fn _part_1(_input: &[String]) -> u32 {
    let board: Vec<Vec<char>> = _input.iter().map(|x| x.chars().collect()).collect();
    calculate_energy(&board, (0, 0), Direction::Right)
}

fn _part_2(_input: &[String]) -> u32 {
    let board: Vec<Vec<char>> = _input.iter().map(|x| x.chars().collect()).collect();
    let start_locations: Vec<_> = (0..board.len())
        .flat_map(|x| {
            vec![
                (x, 0, Direction::Down),
                (x, board[0].len() - 1, Direction::Up),
            ]
        })
        .chain((0..board[0].len()).flat_map(|y| {
            vec![
                (0, y, Direction::Right),
                (board.len() - 1, y, Direction::Left),
            ]
        }))
        .collect();
    start_locations
        .par_iter()
        .map(|(x, y, direction)| calculate_energy(&board, (*x, *y), *direction))
        .max()
        .unwrap()
}
