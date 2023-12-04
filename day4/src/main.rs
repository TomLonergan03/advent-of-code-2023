use regex::Regex;
use std::{
    collections::HashSet,
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

fn get_card_values(card: &str) -> (HashSet<u32>, HashSet<u32>) {
    let clean_card = Regex::new(r"\ {2,}").unwrap().replace_all(card, " ");
    let parts = clean_card.split(" | ").collect::<Vec<&str>>();
    let my_numbers: HashSet<u32> = parts[0]
        .split(' ')
        .skip(2)
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let winning_numbers: HashSet<u32> = parts[1]
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    (my_numbers, winning_numbers)
}

fn part_1(input: &[String]) -> usize {
    input
        .iter()
        .map(|x| get_card_values(x))
        .fold(0, |acc, (my_numbers, winning_numbers)| {
            let number_of_winners = my_numbers.intersection(&winning_numbers).count();
            if number_of_winners == 0 {
                acc
            } else {
                acc + 2_usize.pow((number_of_winners - 1).try_into().unwrap())
            }
        })
}

struct Card {
    count: i32,
    winners: usize,
}

fn part_2(input: &[String]) -> i32 {
    let mut cards: Vec<Card> = input
        .iter()
        .map(|x| Card {
            count: 1,
            winners: {
                let (my_numbers, winning_numbers) = get_card_values(x);
                my_numbers.intersection(&winning_numbers).count()
            },
        })
        .collect();
    for i in 0..cards.len() {
        let number_of_winners = cards[i].winners;
        if number_of_winners > 0 {
            for j in 1..=number_of_winners {
                cards[i + j].count += cards[i].count;
            }
        }
    }
    cards.iter().fold(0, |acc, card| acc + card.count)
}
