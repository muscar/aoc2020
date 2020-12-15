use crate::part::Part;

use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

fn play(xs: &[i64], target: i64) -> i64 {
    let mut seen = HashMap::new();
    seen.extend(
        xs.iter()
            .enumerate()
            .map(|(turn, &x)| (x, (-1, turn as i64 + 1))),
    );
    let mut last = xs[xs.len() - 1];
    let mut turn = xs.len() as i64 + 1;
    while turn <= target {
        last = match seen.get(&last) {
            Some((-1, _)) => 0,
            Some((t1, t2)) => t2 - t1,
            None => last,
        };
        if let Some((t1, t2)) = seen.get_mut(&last) {
            *t1 = *t2;
            *t2 = turn;
        } else {
            seen.insert(last, (-1, turn));
        }
        turn += 1;
    }
    last
}

fn part1(xs: &[i64]) -> i64 {
    play(xs, 2020)
}

fn part2(xs: &[i64]) -> i64 {
    play(xs, 30000000)
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let input = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&input),
        Part::Part2 => part2(&input),
    }
}
