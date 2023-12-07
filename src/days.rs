mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn run_day(day_number: u8, input: Vec<String>) {
    match day_number {
        1 => day01::_run(input),
        2 => day02::_run(input),
        3 => day03::_run(input),
        4 => day04::_run(input),
        // 5 => day05::_run(input),
        6 => day06::_run(input),
        7 => day07::_run(input),
        // 8 => day08::_run(input),
        // 9 => day09::_run(input),
        // 10 => day10::_run(input),
        // 11 => day11::_run(input),
        // 12 => day12::_run(input),
        // 13 => day13::_run(input),
        // 14 => day14::_run(input),
        // 15 => day15::_run(input),
        // 16 => day16::_run(input),
        // 17 => day17::_run(input),
        // 18 => day18::_run(input),
        // 19 => day19::_run(input),
        // 20 => day20::_run(input),
        // 21 => day21::_run(input),
        // 22 => day22::_run(input),
        // 23 => day23::_run(input),
        // 24 => day24::_run(input),
        // 25 => day25::_run(input),
        _ => {
            println!("Day {} not implemented", day_number);
        }
    }
}
