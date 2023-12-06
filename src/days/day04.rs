use std::collections::HashSet;

use regex::Regex;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
