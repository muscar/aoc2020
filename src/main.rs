use std::env;

mod day1;
mod day2;
mod day3;
mod part;

fn main() {
    let days: Vec<&dyn Fn(part::Part, &str) -> i64> = vec![&day1::run, &day2::run, &day3::run];

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
