pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn find_next(sequence: Vec<i32>) -> i32 {
    let mut steps = vec![sequence];
    while steps.last().unwrap().iter().any(|&x| x != 0) {
        steps.push(
            steps
                .last()
                .unwrap()
                .iter()
                .zip(steps.last().unwrap().iter().skip(1))
                .map(|(x, y)| y - x)
                .collect(),
        )
    }
    for i in (0..steps.len() - 1).rev() {
        let new_value = steps[i + 1].last().unwrap() + steps[i].last().unwrap();
        steps[i].push(new_value)
    }
    *steps[0].last().unwrap()
}

fn _part_1(_input: &[String]) -> i32 {
    _input
        .iter()
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .fold(0, |acc, x| acc + find_next(x))
}

fn _part_2(_input: &[String]) -> i32 {
    _input
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse().unwrap())
                .rev()
                .collect()
        })
        .fold(0, |acc, x| acc + find_next(x))
}
