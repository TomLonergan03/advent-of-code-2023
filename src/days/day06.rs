pub fn run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn _part_1(_input: &[String]) -> i32 {
    let times: Vec<i32> = _input[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<i32> = _input[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    times
        .iter()
        .zip(distances)
        .fold(1, |acc, (time, distance)| {
            let min = (0..*time).find(|t| t * (time - t) > distance).unwrap();
            let max = (0..*time)
                .rev()
                .find(|t| t * (time - t) > distance)
                .unwrap();
            acc * (max - min + 1)
        })
}

fn _part_2(_input: &[String]) -> u64 {
    let time = _input[0]
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance = _input[1]
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    let min = (0..time).find(|t| t * (time - t) > distance).unwrap();
    let max = (0..time).rev().find(|t| t * (time - t) > distance).unwrap();
    max - min + 1
}
