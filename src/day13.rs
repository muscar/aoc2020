use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(earliest: i64, ids: &[i64]) -> i64 {
    let (id, departure) = ids
        .iter()
        .map(|id| (id, id * (earliest / id + 1)))
        .min_by(|(_, t1), (_, t2)| t1.cmp(t2))
        .unwrap();
    id * (departure - earliest)
}

// Chinese remainder theorem - taken from Rosetta Code

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

//

fn part2(ids: &[i64]) -> i64 {
    let congruences = ids
        .iter()
        .enumerate()
        .filter(|(_, &n)| n > 0)
        .map(|(i, n)| (n - i as i64, n))
        .collect::<Vec<_>>();

    let modulii = congruences.iter().map(|(_, &m)| m).collect::<Vec<_>>();
    let residues = congruences
        .iter()
        .map(|(r, _)| *r as i64)
        .collect::<Vec<_>>();

    chinese_remainder(&residues, &modulii).unwrap()
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let input = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let earliest = input[0].parse().unwrap();
    match part {
        Part::Part1 => part1(
            earliest,
            &input[1]
                .split(',')
                .filter(|&s| s != "x")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>(),
        ) as i64,
        Part::Part2 => part2(
            &input[1]
                .split(',')
                .map(|s| if s != "x" { s.parse().unwrap() } else { 0 })
                .collect::<Vec<_>>(),
        ) as i64,
    }
}
