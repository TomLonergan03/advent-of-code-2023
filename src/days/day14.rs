use std::collections::HashSet;

use crate::utils::transpose;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

type Platform = Vec<Vec<char>>;

fn determine_load(platform: &[Vec<char>]) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .rev()
        .flat_map(|(i, x)| {
            x.iter()
                .map(|&y| if y != 'O' { 0 } else { i + 1 })
                .collect::<Vec<_>>()
        })
        .sum()
}

fn roll_north(platform: Platform) -> Platform {
    let mut platform = platform.clone();
    for i in 0..platform[0].len() {
        for j in 0..platform.len() {
            if platform[j][i] != 'O' {
                continue;
            }
            for k in (0..j).rev() {
                if platform[k][i] != '.' {
                    platform[j][i] = '.';
                    platform[k + 1][i] = 'O';
                    break;
                }
                if k == 0 {
                    platform[j][i] = '.';
                    platform[k][i] = 'O';
                }
            }
        }
    }
    platform
}

fn _part_1(_input: &[String]) -> usize {
    let platform = _input
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let platform = roll_north(platform);
    determine_load(&platform)
}

fn roll_south(platform: Platform) -> Platform {
    roll_north(platform.iter().cloned().rev().collect())
        .iter()
        .cloned()
        .rev()
        .collect()
}

fn roll_east(platform: Platform) -> Platform {
    transpose(&roll_north(
        transpose(&platform).iter().rev().cloned().collect(),
    ))
    .iter()
    .map(|x| x.iter().cloned().rev().collect::<Vec<_>>())
    .collect()
}

fn roll_west(platform: Platform) -> Platform {
    transpose(&roll_north(
        transpose(&platform)
            .iter()
            .map(|x| x.iter().cloned().rev().collect())
            .collect::<Vec<_>>(),
    ))
    .iter()
    .cloned()
    .rev()
    .collect()
}

fn spin_cycle(platform: Platform, number_of_cycles: usize) -> Platform {
    let mut platform = platform;
    let mut seen_before: HashSet<Vec<Vec<char>>> = std::collections::HashSet::new();
    let mut cycle_started = false;
    let mut start_point = 0;
    for i in 0..number_of_cycles {
        platform = roll_east(roll_south(roll_west(roll_north(platform))));
        if seen_before.contains(&platform) && cycle_started {
            break;
        }
        if seen_before.contains(&platform) {
            start_point = i;
            cycle_started = true;
            seen_before.clear()
        }
        seen_before.insert(platform.clone());
    }
    let cycle_length = seen_before.len();
    let cycles_remaining = (number_of_cycles - start_point - 1) % cycle_length;
    for _ in 0..cycles_remaining {
        platform = roll_east(roll_south(roll_west(roll_north(platform))));
    }
    platform
}

fn _part_2(_input: &[String]) -> usize {
    let platform = _input
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let platform = spin_cycle(platform, 1000000000);
    determine_load(&platform)
}
