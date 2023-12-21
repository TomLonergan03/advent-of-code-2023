use crate::utils::{get_neighbours, NeighbourArrangement, NeighbourPattern};

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> u32 {
    let mut sum = 0;
    let chars = input
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for i in 0..chars.len() {
        for j in 0..chars[0].len() {
            if chars[i][j] == '.' {
                continue;
            }
            if !chars[i][j].is_numeric() {
                let neighbours = get_neighbours(
                    i,
                    j,
                    chars.len(),
                    chars[0].len(),
                    NeighbourPattern::All,
                    NeighbourArrangement::Scanning,
                );
                let mut can_start_num = 0;
                let mut current_row = 0;
                for (x, y) in neighbours {
                    if x > current_row {
                        current_row = x;
                        can_start_num = 0;
                    }
                    if chars[x][y].is_numeric() && (y >= can_start_num || x > current_row) {
                        let (number, end) = build_num(&chars, x, y);
                        can_start_num = end;
                        sum += number
                    }
                }
            }
        }
    }
    sum
}

fn build_num(array: &[Vec<char>], i: usize, j: usize) -> (u32, usize) {
    let mut start = j;
    while start > 0 {
        if !array[i][start].is_numeric() {
            break;
        }
        start -= 1;
    }
    if !array[i][start].is_numeric() {
        start += 1;
    }
    let mut result = 0;
    let mut end = start;
    while array[i][end].is_numeric() {
        result *= 10;
        result += array[i][end].to_digit(10).unwrap();
        end += 1;
        if end == array[0].len() {
            break;
        }
    }
    (result, end)
}

fn part_2(_input: &[String]) -> u32 {
    let mut sum = 0;
    let chars = _input
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for i in 0..chars.len() {
        for j in 0..chars[0].len() {
            if chars[i][j] == '.' {
                continue;
            }
            if chars[i][j] == '*' {
                let neighbours = get_neighbours(
                    i,
                    j,
                    chars.len(),
                    chars[0].len(),
                    NeighbourPattern::All,
                    NeighbourArrangement::Scanning,
                );
                let mut can_start_num = 0;
                let mut current_row = 0;
                let mut surrounding_nums = Vec::new();
                for (x, y) in neighbours {
                    if x > current_row {
                        current_row = x;
                        can_start_num = 0;
                    }
                    if chars[x][y].is_numeric() && (y >= can_start_num || x > current_row) {
                        let (number, end) = build_num(&chars, x, y);
                        can_start_num = end;
                        surrounding_nums.push(number)
                    }
                }
                if surrounding_nums.len() == 2 {
                    sum += surrounding_nums[0] * surrounding_nums[1]
                }
            }
        }
    }
    sum
}
