use crate::{part::Part, utils::parse_seq};

use std::{
    fmt::Debug,
    io::{BufRead, BufReader},
};
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
        match s.split(':').map(|s| s.trim()).collect::<Vec<&str>>()[..] {
            [policy, password] => Ok(Self::new(
                policy.parse().expect("failed to parse policy"),
                password.to_string(),
            )),
            _ => Err("failed to parse entry"),
        }
    }
}

#[derive(Debug)]
struct Policy {
    start: usize,
    end: usize,
    character: char,
}

impl Policy {
    fn new(start: usize, end: usize, character: char) -> Self {
        Self {
            start,
            end,
            character,
        }
    }
}

impl FromStr for Policy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').map(|s| s.trim()).collect::<Vec<&str>>()[..] {
            [range, s] if s.len() == 1 => match parse_seq(range.split('-'))[..] {
                [start, end] => Ok(Self::new(start, end, s.chars().next().unwrap())),
                _ => Err("failed to parse range"),
            },
            _ => Err("failed to parse policy"),
        }
    }
}

fn count_if<P>(entries: &[Entry], p: P) -> usize
where
    P: Fn(&Entry) -> bool,
{
    entries.iter().filter(|e| p(*e)).count()
}

fn part1(entries: &[Entry]) -> usize {
    count_if(entries, |e| {
        let cnt = e
            .password
            .chars()
            .filter(|c| *c == e.policy.character)
            .count();
        e.policy.start <= cnt && cnt <= e.policy.end
    })
}

fn part2(entries: &[Entry]) -> usize {
    count_if(entries, |e| {
        let c1 = e
            .password
            .chars()
            .nth(e.policy.start - 1)
            .expect("password too short");
        let c2 = e
            .password
            .chars()
            .nth(e.policy.end - 1)
            .expect("password too short");
        c1 == e.policy.character && c2 != e.policy.character
            || c1 != e.policy.character && c2 == e.policy.character
    })
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
        Part::Part1 => part1(&entries) as i64,
        Part::Part2 => part2(&entries) as i64,
    }
}
