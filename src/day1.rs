pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> i32 {
    let mut result: i32 = 0;
    let regex = regex::Regex::new(r"[0-9]").unwrap();
    for line in input.iter() {
        let mut line_val = 0;
        for c in line.chars() {
            if regex.is_match(&c.to_string()) {
                line_val += (c.to_digit(10).unwrap() * 10) as i32;
                break;
            }
        }
        for c in line.chars().rev() {
            if regex.is_match(&c.to_string()) {
                line_val += c.to_digit(10).unwrap() as i32;
                break;
            }
        }
        result += line_val;
    }
    result
}

fn part_2(input: &[String]) -> i32 {
    let options = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let mut result: i32 = 0;
    for line in input.iter() {
        let mut matches = Vec::new();
        for i in 0..line.len() {
            for word in options.iter() {
                if line[i..].starts_with(word) {
                    matches.push(convert(word));
                }
            }
        }
        result += matches[0] * 10 + matches[matches.len() - 1];
    }
    result
}

fn convert(word: &str) -> i32 {
    if word.len() == 1 {
        return word.parse::<i32>().unwrap();
    }
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => panic!("Invalid word: {}", x),
    }
}
