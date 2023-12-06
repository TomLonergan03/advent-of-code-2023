use std::{env::args, time::Instant};

mod day1;
mod day2;
mod day3;
mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
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
        let _ = run_task(parsed_args.day, &parsed_args);
        println!("{SEPARATOR}");
        return;
    }
    for i in 1..=25 {
        let result = run_task(i, &parsed_args);
        println!("{SEPARATOR}");
        if result.is_err_and(|x| x.to_string() == "Day not released yet") {
            break;
        }
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

fn run_task(day_number: u8, args: &ParsedArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day {}", day_number);
    let input = input_handling::get_input(day_number, args.use_test)?;
    let now = Instant::now();
    match day_number {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        // 5 => day5::run(input),
        // 6 => day6::run(input),
        // 7 => day7::run(input),
        // 8 => day8::run(input),
        // 9 => day9::run(input),
        // 10 => day10::run(input),
        // 11 => day11::run(input),
        // 12 => day12::run(input),
        // 13 => day13::run(input),
        // 14 => day14::run(input),
        // 15 => day15::run(input),
        // 16 => day16::run(input),
        // 17 => day17::run(input),
        // 18 => day18::run(input),
        // 19 => day19::run(input),
        // 20 => day20::run(input),
        // 21 => day21::run(input),
        // 22 => day22::run(input),
        // 23 => day23::run(input),
        // 24 => day24::run(input),
        // 25 => day25::run(input),
        _ => {
            println!("Day {} not implemented", day_number);
            return Ok(());
        }
    }
    println!("Day {} completed in: {:.2?}", day_number, now.elapsed());
    Ok(())
}
