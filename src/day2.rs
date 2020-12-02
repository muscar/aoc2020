use crate::part::Part;
use std::io::{BufRead, BufReader};
use std::{fs::File, str::FromStr};

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn new(policy: Policy, password: String) -> Self {
        Self { policy, password }
    }
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').map(|s| s.trim()).collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("failed to parse entry");
        }
        Ok(Self::new(
            parts[0].parse().expect("failed to parse policy"),
            parts[1].to_string(),
        ))
    }
}

#[derive(Debug)]
struct Policy {
    lower_bound: usize,
    upper_bount: usize,
    character: char,
}

impl Policy {
    fn new(lower_bound: usize, upper_bount: usize, character: char) -> Self {
        Self {
            lower_bound,
            upper_bount,
            character,
        }
    }
}

impl FromStr for Policy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').map(|s| s.trim()).collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("failed to parse policy");
        }
        let range = parts[0]
            .split('-')
            .map(|s| s.trim().parse().expect("failed to parse number"))
            .collect::<Vec<usize>>();
        if range.len() != 2 {
            return Err("failed to parse range");
        }
        if parts[1].len() != 1 {
            return Err("failed to parse char");
        }
        Ok(Self::new(
            range[0],
            range[1],
            parts[1].chars().nth(0).unwrap(),
        ))
    }
}

fn part1(entries: &[Entry]) -> i64 {
    let mut conforming = 0;
    for e in entries.iter() {
        let mut cnt = 0;
        for c in e.password.chars() {
            if c == e.policy.character {
                cnt += 1;
            }
        }
        if e.policy.lower_bound <= cnt && cnt <= e.policy.upper_bount {
            conforming += 1;
        }
    }
    conforming
}

fn part2(entries: &[Entry]) -> i64 {
    let mut conforming = 0;
    for e in entries.iter() {
        let c1 = e
            .password
            .chars()
            .nth(e.policy.lower_bound - 1)
            .expect("password too short");
        let c2 = e
            .password
            .chars()
            .nth(e.policy.upper_bount - 1)
            .expect("password too short");
        if c1 == e.policy.character && c2 != e.policy.character
            || c1 != e.policy.character && c2 == e.policy.character
        {
            conforming += 1;
        }
    }
    conforming
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let entries = reader
        .lines()
        .map(|s| {
            s.expect("failed to read line")
                .parse()
                .expect("failed to parse entry")
        })
        .collect::<Vec<Entry>>();
    match part {
        Part::Part1 => part1(&entries),
        Part::Part2 => part2(&entries),
    }
}
