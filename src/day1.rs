use crate::{part::Part, utils::parse_seq};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(ns: &[i64], target: i64) -> Option<i64> {
    let mut l = 0;
    let mut r = ns.len() - 1;
    while l < r {
        if ns[l] + ns[r] == target {
            return Some(ns[l] * ns[r]);
        } else if target - ns[r] > ns[l] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    None
}

fn part2(ns: &[i64], target: i64) -> Option<i64> {
    for i in 0..ns.len() - 1 {
        let mut l = i + 1;
        let mut r = ns.len() - 1;
        while l < r {
            if ns[l] + ns[r] + ns[i] == target {
                return Some(ns[l] * ns[r] * ns[i]);
            } else if target - ns[r] - ns[l] > ns[i] {
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
    let mut ns = parse_seq(reader.lines().map(|s| s.expect("failed to read line")));
    ns.sort_unstable();
    match part {
        Part::Part1 => part1(&ns, 2020).unwrap(),
        Part::Part2 => part2(&ns, 2020).unwrap(),
    }
}
