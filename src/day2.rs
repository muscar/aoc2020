use crate::{
    part::Part,
    utils::{count_if, parse_lines, parse_seq, split_trim},
};

use std::{fmt::Debug, io::BufReader};
use std::{fs::File, str::FromStr};

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match split_trim(s, ':')[..] {
            [policy, password] => Ok(Self {
                policy: policy.parse().expect("failed to parse policy"),
                password: password.to_string(),
            }),
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

impl FromStr for Policy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match split_trim(s, ' ')[..] {
            [range, s] if s.len() == 1 => match parse_seq(range.split('-'))[..] {
                [start, end] => Ok(Self {
                    start,
                    end,
                    character: s.chars().next().unwrap(),
                }),
                _ => Err("failed to parse range"),
            },
            _ => Err("failed to parse policy"),
        }
    }
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
    let entries = parse_lines(reader);
    match part {
        Part::Part1 => part1(&entries) as i64,
        Part::Part2 => part2(&entries) as i64,
    }
}
