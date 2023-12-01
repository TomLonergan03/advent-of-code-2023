use std::{
    env::args,
    fs::File,
    io::{self, BufRead},
    path::Path,
    process::exit,
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let args = args().collect::<Vec<String>>();
    let path = Path::new("input.txt");
    // let path = Path::new("test.txt");
    let input: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| line.expect("Line read failure"))
        .collect::<Vec<String>>();
    if args[1] == "1" {
        println!("Part 1: {}", part_1(&input));
    } else if args[1] == "2" {
        println!("Part 2: {}", part_2(&input));
    } else {
        println!("Invalid part number");
        exit(1);
    }
    println!("In: {:.2?}", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
