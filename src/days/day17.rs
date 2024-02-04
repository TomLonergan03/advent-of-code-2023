use std::collections::VecDeque;

use crate::utils::Direction;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn get_valid_moves(
    city: &[Vec<u8>],
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    direction: Direction,
) -> Vec<(usize, usize, usize, Direction)> {
    let (x, y, n, m) = (x as i32, y as i32, n as i32, m as i32);
    let mut valid = vec![];
    match direction {
        Direction::Up | Direction::Down => {
            for i in -3..=3 {
                if i == 0 {
                    continue;
                }
                if let Some(j) = y.checked_add(i) {
                    if j >= m {
                        continue;
                    }
                    valid.push((
                        x as usize,
                        j as usize,
                        i as usize,
                        if i < 0 {
                            Direction::Left
                        } else {
                            Direction::Right
                        },
                    ))
                }
            }
        }
        Direction::Left | Direction::Right => {
            for i in -3..=3 {
                if i == 0 {
                    continue;
                }
                if let Some(j) = x.checked_add(i) {
                    if j >= n {
                        continue;
                    }
                    valid.push((
                        j as usize,
                        y as usize,
                        i as usize,
                        if i < 0 {
                            Direction::Up
                        } else {
                            Direction::Down
                        },
                    ))
                }
            }
        }
        Direction::None => {
            for i in 1..=3 {
                if let Some(j) = x.checked_add(i) {
                    if j < n - 1 {
                        valid.push((j as usize, y as usize, i as usize, Direction::Down))
                    }
                }
                if let Some(j) = x.checked_sub(i) {
                    valid.push((j as usize, y as usize, i as usize, Direction::Up))
                }
            }
        }
    }
    valid
}

fn _part_1(_input: &[String]) -> u32 {
    let city: Vec<Vec<u8>> = _input
        .iter()
        .map(|x| x.chars().map(|y| y.to_string().parse().unwrap()).collect())
        .collect();
    let mut best_distance_to: Vec<Vec<u32>> = Vec::new();
    let mut to_visit: VecDeque<(usize, usize, Direction)> = VecDeque::new();
    to_visit.push_back((0, 0, Direction::None));
    while !to_visit.is_empty() {
        let (x, y, direction) = to_visit.pop_front().unwrap();
        let next_locations = get_valid_moves(&city, x, y, city.len(), city[0].len(), direction);
        for (new_x, new_y, heat_loss, location) in next_locations {}
    }

    *best_distance_to.last().unwrap().last().unwrap()
}

fn _part_2(_input: &[String]) -> i32 {
    -1
}
