use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod part;

fn main() {
    let days: Vec<&dyn Fn(part::Part, &str) -> i64> = vec![
        &day1::run,
        &day2::run,
        &day3::run,
        &day4::run,
        &day5::run,
        &day6::run,
        &day7::run,
        &day8::run,
        &day9::run,
        &day10::run,
        &day11::run,
        &day12::run,
        &day13::run,
        &day14::run,
        &day15::run,
        &day16::run,
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: aoc2020 <day> <input>");
        return;
    }
    let day: usize = args[1].parse().expect("failed to parse the day");

    println!("Day #{}", day);
    println!(
        "part #1 answer={}",
        days[day - 1](part::Part::Part1, &args[2])
    );
    println!(
        "part #2 answer={}",
        days[day - 1](part::Part::Part2, &args[2])
    );
}
