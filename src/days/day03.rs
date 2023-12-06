use std::cmp::min;

pub fn run(input: Vec<String>) {
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
        let mut can_start_number = true;
        for j in 0..chars[0].len() {
            if chars[i][j] == '.' {
                can_start_number = true;
                continue;
            }
            if chars[i][j].is_numeric() && can_start_number {
                let (result, start, end) = build_num(&chars, i, j);
                // println!("{} {}", start, end);
                for k in i.saturating_sub(1)..min(i + 2, chars.len()) {
                    for l in start.saturating_sub(1)..min(end + 1, chars[0].len()) {
                        if chars[k][l] != '.' && !chars[k][l].is_numeric() {
                            // println!("including {} at {k} {l}", result);
                            sum += result;
                        }
                    }
                }
                can_start_number = false;
            }
        }
    }
    sum
}

fn build_num(array: &[Vec<char>], i: usize, j: usize) -> (u32, usize, usize) {
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
    (result, start, end)
}

fn part_2(_input: &[String]) -> i32 {
    -1
}
