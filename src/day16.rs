use crate::part::Part;

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
use std::{fs::File, ops::RangeInclusive};

#[derive(Debug)]
struct Rule(String, RangeInclusive<i64>, RangeInclusive<i64>);

fn parse_rule(s: &String) -> Rule {
    let mut parts = s.split(": ");
    let (name, ranges) = match (parts.next(), parts.next()) {
        (Some(name), Some(ranges)) => {
            let ranges = ranges
                .split(" or ")
                .map(|r| {
                    match r
                        .split('-')
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        [start, end] => RangeInclusive::new(start.clone(), end.clone()),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>();
            (name, ranges)
        }
        _ => unreachable!(),
    };
    Rule(name.to_string(), ranges[0].clone(), ranges[1].clone())
}

fn part1(rules: &[Rule], others: &[Vec<i64>]) -> i64 {
    let mut sum = 0;
    for t in others {
        for n in t {
            if !rules.iter().any(|r| r.1.contains(&n) || r.2.contains(&n)) {
                sum += n;
            }
        }
    }
    sum
}

fn part2(rules: &[Rule], mine: &[i64], others: &[Vec<i64>]) -> i64 {
    let valid = others
        .iter()
        .filter(|&t| {
            for n in t {
                if !rules.iter().any(|r| r.1.contains(&n) || r.2.contains(&n)) {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<_>>();

    let mut seen = HashMap::new();
    for i in 0..valid[0].len() {
        for r in rules {
            let xs = valid.iter().map(|t| t[i]).collect::<Vec<_>>();
            let is_valid = xs.iter().all(|x| r.1.contains(x) || r.2.contains(x));
            if is_valid {
                if !seen.contains_key(&r.0) {
                    seen.insert(r.0.clone(), vec![]);
                }
                seen.get_mut(&r.0).unwrap().push(i);
            }
        }
    }

    let mut options = seen
        .iter()
        .map(|(r, xs)| (r, xs.iter().cloned().collect::<HashSet<usize>>()))
        .collect::<Vec<_>>();
    options.sort_by(|(_, opts1), (_, opts2)| opts1.len().cmp(&opts2.len()));

    let mut assignment = HashMap::new();
    let mut used = HashSet::new();
    for (r, xs) in options {
        let x = xs.difference(&used).next().unwrap().clone();
        assignment.insert(r, x);
        used.insert(x);
    }

    assignment
        .iter()
        .filter(|(r, _)| r.starts_with("departure"))
        .map(|(_, x)| mine[*x])
        .product()
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut chunks = input.split(String::is_empty);
    let (rules, mine, others) = match (chunks.next(), chunks.next(), chunks.next()) {
        (Some(rules), Some(mine), Some(others)) => {
            let rules = rules.iter().map(parse_rule).collect::<Vec<_>>();
            (
                rules,
                mine.iter()
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
                others
                    .iter()
                    .skip(1)
                    .map(|s| {
                        s.split(',')
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        }
        _ => unreachable!(),
    };
    match part {
        Part::Part1 => part1(&rules, &others),
        Part::Part2 => part2(&rules, &mine, &others),
    }
}
