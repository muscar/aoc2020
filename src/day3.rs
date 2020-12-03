use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};

const TREE: u8 = b'#';

fn count_trees(map: &[String], dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut cnt = 0;

    while y < map.len() {
        if map[y].as_bytes()[x] == TREE {
            cnt += 1;
        }
        x = (x + dx) % map[y].len();
        y += dy;
    }

    cnt
}

fn part1(map: &[String]) -> usize {
    count_trees(map, 3, 1)
}

fn part2(map: &[String]) -> usize {
    let deltas = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    deltas
        .iter()
        .map(|(dx, dy)| count_trees(map, *dx, *dy))
        .product()
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let map = reader
        .lines()
        .map(|l| l.expect("failed to read line"))
        .collect::<Vec<String>>();
    match part {
        Part::Part1 => part1(&map) as i64,
        Part::Part2 => part2(&map) as i64,
    }
}
