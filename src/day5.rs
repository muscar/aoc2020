use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn partition(bs: &[bool]) -> i64 {
    bs.iter()
        .zip((0..bs.len()).rev())
        .map(|(&v, w)| if v { 1 << w } else { 0 })
        .sum()
}

fn seat_id(s: &str) -> i64 {
    let (l, r) = s.split_at(7);
    let row = partition(&l.as_bytes().iter().map(|&b| b == b'B').collect::<Vec<_>>());
    let seat = partition(&r.as_bytes().iter().map(|&b| b == b'R').collect::<Vec<_>>());
    row * 8 + seat
}

fn part1(xs: &[String]) -> i64 {
    xs.iter().map(|x| seat_id(x)).max().unwrap()
}

fn part2(xs: &[String]) -> i64 {
    let mut ids = xs.iter().map(|x| seat_id(x)).collect::<Vec<i64>>();
    ids.sort_unstable();
    let p = ids
        .iter()
        .zip(ids.iter().skip(1))
        .find(|(&x, &y)| y - x > 1)
        .unwrap();
    p.0 + 1
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let xs = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&xs),
        Part::Part2 => part2(&xs),
    }
}
