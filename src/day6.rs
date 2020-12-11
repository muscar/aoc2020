use crate::part::Part;

use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

fn part1<It: Iterator<Item = String>>(it: It) -> usize {
    let mut cnt = 0;
    let mut group = HashSet::new();
    for l in it {
        if l.is_empty() {
            cnt += group.len();
            group.clear();
        } else {
            for c in l.chars() {
                group.insert(c);
            }
        }
    }
    if !group.is_empty() {
        cnt += group.len();
    }
    cnt
}

fn part2<It: Iterator<Item = String>>(it: It) -> usize {
    let mut cnt = 0;
    let mut group = HashSet::new();
    let mut new_group = true;
    for l in it {
        if l.is_empty() {
            cnt += group.len();
            group.clear();
            new_group = true;
        } else if new_group {
            for c in l.chars() {
                group.insert(c);
            }
            new_group = false;
        } else {
            let xs: HashSet<char> = l.chars().collect();
            group = group.intersection(&xs).copied().collect();
        }
    }
    if !group.is_empty() {
        cnt += group.len();
    }
    cnt
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    match part {
        Part::Part1 => part1(reader.lines().map(|l| l.unwrap())) as i64,
        Part::Part2 => part2(reader.lines().map(|l| l.unwrap())) as i64,
    }
}
