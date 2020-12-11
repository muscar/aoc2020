use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(ns: &[i64], preamble: usize) -> i64 {
    ns.windows(preamble + 1)
        .find_map(|x| {
            let (&target, ns) = x.split_last().unwrap();
            for i in 0..ns.len() - 1 {
                for j in i + 1..ns.len() {
                    if ns[i] + ns[j] == target && ns[i] != ns[j] {
                        return None;
                    }
                }
            }
            Some(target)
        })
        .unwrap()
}

fn part2(ns: &[i64], target: i64) -> i64 {
    for i in 0..ns.len() - 1 {
        let mut acc = ns[i];
        let mut off = 1;
        while acc < target {
            acc += ns[i + off];
            off = off + 1;
        }
        if acc == target {
            let mut aux = ns[i..i + off].iter().collect::<Vec<_>>();
            aux.sort_unstable();
            return aux[0] + aux[aux.len() - 1];
        }
    }
    0
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let ns = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .map(|l| l.parse().expect("failed to parse entry"))
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&ns, 25),
        Part::Part2 => part2(&ns, 248131121),
    }
}
