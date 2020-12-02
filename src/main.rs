use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: aoc2020 <day> <input>");
        return;
    }

    println!("part1 answer={}", day1::run(day1::Part::Part1, &args[2]));
    println!("part1 answer={}", day1::run(day1::Part::Part2, &args[2]));
}
