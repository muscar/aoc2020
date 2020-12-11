use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(ns: &[usize]) -> usize {
    let mut ones = 1;
    let mut threes = 1;
    for i in 0..ns.len() - 1 {
        match ns[i + 1] - ns[i] {
            1 => ones += 1,
            3 => threes += 1,
            _ => panic!(),
        }
    }
    threes * ones
}

fn part2(ns: &[usize]) -> usize {
    let ns = vec![0].iter().chain(ns).cloned().collect::<Vec<_>>();
    ns.iter()
        .zip(ns.iter().skip(1))
        .map(|(m, n)| n - m)
        .fold((0, 0, 1), |(a, b, c), d| match d {
            1 => (b, c, a + b + c),
            2 => (c, 0, b + c),
            3 => (0, 0, c),
            _ => unreachable!(),
        })
        .2
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let mut ns = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .map(|l| l.parse().expect("failed to parse entry"))
        .collect::<Vec<usize>>();
    ns.sort_unstable();
    match part {
        Part::Part1 => part1(&ns) as i64,
        Part::Part2 => part2(&ns) as i64,
    }
}
