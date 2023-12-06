use std::{env::args, time::Instant};

use chrono::Datelike;

mod days;
mod input_handling;
mod utils;

#[derive(PartialEq, Eq, Hash)]
struct ParsedArgs {
    // 0 represents all days (should be an enum ikik)
    day: u8,
    // 0 represents both parts
    part: u8,
    use_test: bool,
}

const SEPARATOR: &str = "---------------------------------";

fn main() {
    println!("{SEPARATOR}");
    let parsed_args: ParsedArgs = parse_args(args().collect::<Vec<_>>().as_slice());
    if parsed_args.day != 0 {
        run_task(parsed_args.day, &parsed_args);
        println!("{SEPARATOR}");
        return;
    }
    for i in 1..=chrono::offset::Utc::now().day() as u8 {
        run_task(i, &parsed_args);
        println!("{SEPARATOR}");
    }
}

fn parse_args(args: &[String]) -> ParsedArgs {
    if args.len() == 1 {
        return ParsedArgs {
            day: 0,
            part: 0,
            use_test: false,
        };
    }
    if args.len() == 2 {
        return ParsedArgs {
            day: args[1].get(1..).unwrap().parse().unwrap(),
            part: 0,
            use_test: false,
        };
    }
    ParsedArgs {
        day: args[1].get(1..2).unwrap().parse().unwrap(),
        part: args[2].get(1..2).unwrap().parse().unwrap(),
        use_test: if args.len() == 4 {
            args[3].parse().unwrap()
        } else {
            false
        },
    }
}

fn run_task(day_number: u8, args: &ParsedArgs) {
    println!("Day {}", day_number);
    let input = input_handling::get_input(day_number, args.use_test).unwrap();
    let now = Instant::now();
    days::run_day(day_number, input);
    println!("Day {} completed in: {:.2?}", day_number, now.elapsed());
}
