use crate::part::Part;

use std::io::{BufRead, BufReader};
use std::{collections::BTreeMap, fs::File};

#[derive(Debug)]
enum Instr {
    SetMask(Vec<(usize, i64)>),
    Write(i64, i64),
}

fn decode_v1_mask(mask: &[(usize, i64)]) -> (i64, i64) {
    let base: i64 = 2;
    mask.iter().filter(|(_, c)| *c >= 0).fold(
        (base.pow(36) - 1, 0),
        |(and_mask, or_mask), (w, b)| match *b {
            0 => (and_mask - base.pow(*w as u32), or_mask),
            _ => (and_mask, or_mask + base.pow(*w as u32)),
        },
    )
}

fn part1(prog: &[Instr]) -> i64 {
    let base: i64 = 2;
    let (acc, _) = prog.iter().fold(
        (BTreeMap::new(), (base.pow(36) - 1, 0)),
        |(mut acc, (and_mask, or_mask)), instr| match instr {
            Instr::SetMask(mask) => (acc, decode_v1_mask(&mask)),
            Instr::Write(addr, val) => {
                acc.insert(addr, val & and_mask | or_mask);
                (acc, (and_mask, or_mask))
            }
        },
    );
    acc.iter()
        .filter_map(|(_, &v)| if v > 0 { Some(v) } else { None })
        .sum()
}

fn apply_v2_mask(addr: i64, mask: &[(usize, i64)]) -> Vec<i64> {
    let base: i64 = 2;
    let addr = mask
        .iter()
        .filter(|(_, b)| *b == 1)
        .fold(addr, |a, (w, _)| a | base.pow(*w as u32));

    let mut acc = vec![];
    let xs = mask
        .iter()
        .filter_map(|(w, b)| if *b < 0 { Some(*w) } else { None })
        .collect::<Vec<_>>();
    let mut i = 0i64;
    let mut s = vec![-1; xs.len()];
    while i >= 0 {
        if i as usize == s.len() {
            let mask = xs
                .iter()
                .cloned()
                .zip(s.iter().cloned())
                .collect::<Vec<_>>();
            let (and_mask, or_mask) = decode_v1_mask(&mask);
            acc.push(addr & and_mask | or_mask);
            i -= 1;
        }
        if s[i as usize] < 1 {
            s[i as usize] += 1;
            i += 1;
        } else {
            s[i as usize] = -1;
            i -= 1;
        }
    }

    acc
}

fn part2(prog: &[Instr]) -> i64 {
    let (acc, _) = prog.iter().fold(
        (BTreeMap::new(), vec![]),
        |(mut acc, mask), instr| match instr {
            Instr::SetMask(mask) => (acc, mask.clone()),
            Instr::Write(addr, val) => {
                for a in apply_v2_mask(*addr, &mask) {
                    acc.insert(a, val);
                }
                (acc, mask)
            }
        },
    );
    acc.iter()
        .filter_map(|(_, &v)| if *v > 0 { Some(v) } else { None })
        .sum()
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let input = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .map(|l| {
            if l.starts_with("mask") {
                let mask = l
                    .split(" = ")
                    .nth(1)
                    .unwrap()
                    .chars()
                    .map(|c| match c {
                        '0' => 0,
                        '1' => 1,
                        _ => -1,
                    })
                    .rev()
                    .enumerate()
                    .collect::<Vec<_>>();
                Instr::SetMask(mask)
            } else if l.starts_with("mem") {
                let mut parts = l.split(" = ");
                match (parts.next(), parts.next()) {
                    (Some(loc), Some(val)) => {
                        Instr::Write(loc[4..loc.len() - 1].parse().unwrap(), val.parse().unwrap())
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!();
            }
        })
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&input),
        Part::Part2 => part2(&input),
    }
}
