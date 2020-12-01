use std::fs::File;
use std::io::{BufReader, BufRead};

pub enum Part {
    Part1,
    Part2
}

pub fn part1(mut ns: Vec<i64>) -> Option<i64> {
    ns.sort();
    let mut l = 0;
    let mut r = ns.len() - 1;
    while l < r {
        if ns[l] + ns[r] == 2020 {
            return Some(ns[l] * ns[r]);
        } else if 2020 - ns[r] > ns[l] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    None
}

pub fn part2(mut ns: Vec<i64>) -> Option<i64> {
    ns.sort();
    for i in 0..ns.len() - 1 {
        let mut l = i + 1;
        let mut r = ns.len() - 1;
        while l < r {
            if ns[l] + ns[r] + ns[i] == 2020 {
                return Some(ns[l] * ns[r] * ns[i]);
            } else if 2020 - ns[r] - ns[l] > ns[i] {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    None
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let ns = reader.lines().map(|s| s.expect("failed to read line").parse().expect("failed to parse number")).collect::<Vec<i64>>();
    match part {
        Part::Part1 => part1(ns).unwrap(),
        Part::Part2 => part2(ns).unwrap()
    }
}