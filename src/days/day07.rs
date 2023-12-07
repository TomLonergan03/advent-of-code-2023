use std::str::FromStr;

use crate::utils::count_items;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn _part_1(_input: &[String]) -> i32 {
    let mut games: Vec<Hand> = _input.iter().map(|x| x.parse::<Hand>().unwrap()).collect();
    games.sort();
    games
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .fold(0, |acc, (i, x)| acc + i as i32 * x.bid as i32)
}

fn _part_2(_input: &[String]) -> i32 {
    let games: Vec<Hand> = _input.iter().map(|x| x.parse::<Hand>().unwrap()).collect();
    let mut jokerful_games: Vec<Hand> = games.into_iter().map(|x| x.remake_jokerfully()).collect();
    jokerful_games.sort();
    jokerful_games
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .fold(0, |acc, (i, x)| acc + i as i32 * x.bid as i32)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Num(u8),
}

impl FromStr for Card {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            _ => Ok(Card::Num(s.parse::<u8>()?)),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Card::A, Card::A) => std::cmp::Ordering::Equal,
            (Card::A, _) => std::cmp::Ordering::Greater,
            (_, Card::A) => std::cmp::Ordering::Less,
            (Card::K, Card::K) => std::cmp::Ordering::Equal,
            (Card::K, _) => std::cmp::Ordering::Greater,
            (_, Card::K) => std::cmp::Ordering::Less,
            (Card::Q, Card::Q) => std::cmp::Ordering::Equal,
            (Card::Q, _) => std::cmp::Ordering::Greater,
            (_, Card::Q) => std::cmp::Ordering::Less,
            (Card::J, Card::J) => std::cmp::Ordering::Equal,
            (Card::J, _) => std::cmp::Ordering::Greater,
            (_, Card::J) => std::cmp::Ordering::Less,
            (Card::T, Card::T) => std::cmp::Ordering::Equal,
            (Card::T, _) => std::cmp::Ordering::Greater,
            (_, Card::T) => std::cmp::Ordering::Less,
            (Card::Num(x), Card::Num(y)) => x.cmp(y),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u16,
    hand_type: HandType,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u16) -> Self {
        let card_counts = count_items(&cards);
        let mut triple_exists = false;
        let mut number_of_pairs = 0;
        for (_, &count) in card_counts.iter() {
            if count == 5 {
                return Hand {
                    cards,
                    bid,
                    hand_type: HandType::FiveOfAKind,
                };
            }
            if count == 4 {
                return Hand {
                    cards,
                    bid,
                    hand_type: HandType::FourOfAKind,
                };
            }
            if count == 3 {
                triple_exists = true
            }
            if count == 2 {
                number_of_pairs += 1
            }
        }
        match (triple_exists, number_of_pairs) {
            (true, 1) => Hand {
                cards,
                bid,
                hand_type: HandType::FullHouse,
            },
            (true, 0) => Hand {
                cards,
                bid,
                hand_type: HandType::ThreeOfAKind,
            },
            (false, 2) => Hand {
                cards,
                bid,
                hand_type: HandType::TwoPair,
            },
            (false, 1) => Hand {
                cards,
                bid,
                hand_type: HandType::OnePair,
            },
            (false, 0) => Hand {
                cards,
                bid,
                hand_type: HandType::HighCard,
            },
            _ => unreachable!(),
        }
    }

    pub fn remake_jokerfully(self) -> Hand {
        let cards = self.cards;
        let bid = self.bid;
        let card_counts = count_items(&cards);
        let cards = cards
            .iter()
            .map(|&x| if x == Card::J { Card::Num(1) } else { x })
            .collect();
        let mut triple_exists = false;
        let mut number_of_pairs = 0;
        let mut jokers = *card_counts.get(&Card::J).unwrap_or(&0);
        let mut card_counts = card_counts.iter().collect::<Vec<_>>();
        card_counts.sort_by(|(_, count), (_, other_count)| count.cmp(other_count));
        for (_, &count) in card_counts
            .iter()
            .rev()
            .filter(|(&card, _)| card != Card::J)
        {
            let count = count + jokers;
            if count == 5 {
                return Hand {
                    cards,
                    bid,
                    hand_type: HandType::FiveOfAKind,
                };
            }
            if count == 4 {
                return Hand {
                    cards,
                    bid,
                    hand_type: HandType::FourOfAKind,
                };
            }
            if count == 3 {
                triple_exists = true;
                jokers = 0
            }
            if count == 2 {
                number_of_pairs += 1;
                jokers = jokers.saturating_sub(1);
            }
        }
        if number_of_pairs == 0 && !triple_exists && jokers > 1 {
            return match jokers {
                5 => Hand {
                    cards,
                    bid,
                    hand_type: HandType::FiveOfAKind,
                },
                4 => Hand {
                    cards,
                    bid,
                    hand_type: HandType::FourOfAKind,
                },
                3 => Hand {
                    cards,
                    bid,
                    hand_type: HandType::ThreeOfAKind,
                },
                2 => Hand {
                    cards,
                    bid,
                    hand_type: HandType::OnePair,
                },
                _ => unreachable!(),
            };
        }
        match (triple_exists, number_of_pairs) {
            (true, 1) => Hand {
                cards,
                bid,
                hand_type: HandType::FullHouse,
            },
            (true, 0) => Hand {
                cards,
                bid,
                hand_type: HandType::ThreeOfAKind,
            },
            (false, 2) => Hand {
                cards,
                bid,
                hand_type: HandType::TwoPair,
            },
            (false, 1) => Hand {
                cards,
                bid,
                hand_type: HandType::OnePair,
            },
            (false, 0) => Hand {
                cards,
                bid,
                hand_type: HandType::HighCard,
            },
            x => {
                println!("{x:?}");
                unreachable!()
            }
        }
    }
}

impl FromStr for Hand {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .take(5)
            .map(|x| x.to_string().parse::<Card>().unwrap())
            .collect();
        let bid = s.split(' ').nth(1).unwrap().parse::<u16>()?;
        Ok(Hand::new(cards, bid))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (s, o) in self.cards.iter().zip(other.cards.iter()) {
                let ordering = o.cmp(s);
                if ordering != std::cmp::Ordering::Equal {
                    return ordering;
                }
            }
            panic!()
        } else {
            self.hand_type.partial_cmp(&other.hand_type).unwrap()
        }
    }
}
