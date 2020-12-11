use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

const FLOOR: u8 = b'.';
const EMPTY: u8 = b'L';
const OCCUPIED: u8 = b'#';

const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbour(map: &[Vec<u8>], x: usize, y: usize, dx: i32, dy: i32, mut fov: usize) -> Option<u8> {
    let mut i = y as i32 + dy;
    let mut j = x as i32 + dx;
    let mut c = None;
    while fov > 0 && 0 <= i && i < map.len() as i32 && 0 <= j && j < map[i as usize].len() as i32 {
        c = Some(map[i as usize][j as usize]);
        if c.unwrap() != FLOOR {
            break;
        }
        i += dy;
        j += dx;
        fov -= 1;
    }
    c
}

fn step(map: &[Vec<u8>], tolerance: usize, fov: usize, out: &mut [Vec<u8>]) -> bool {
    let mut has_changes = false;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let nearby = DELTAS
                .iter()
                .filter_map(|(dx, dy)| neighbour(map, j, i, *dx, *dy, fov))
                .filter(|&c| c == OCCUPIED)
                .count();
            if map[i][j] == EMPTY && nearby == 0 {
                out[i][j] = OCCUPIED;
                has_changes = true;
            } else if map[i][j] == OCCUPIED && nearby >= tolerance {
                out[i][j] = EMPTY;
                has_changes = true;
            } else {
                out[i][j] = map[i][j];
            }
        }
    }
    has_changes
}

fn steps(mut map: Vec<Vec<u8>>, tolerance: usize, fov: usize) -> usize {
    let mut aux = vec![vec![FLOOR; map[0].len()]; map.len()];
    let mut input = &mut map;
    let mut output = &mut aux;
    while step(input, tolerance, fov, &mut output) {
        swap(&mut input, &mut output);
    }
    return output
        .iter()
        .map(|c| c.iter().filter(|&&c| c == OCCUPIED).count())
        .sum();
}

fn part1(map: Vec<Vec<u8>>) -> usize {
    steps(map, 4, 1)
}

fn part2(map: Vec<Vec<u8>>) -> usize {
    let fov = map.len();
    steps(map, 5, fov)
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let map = reader
        .lines()
        .map(|l| l.expect("failed to read line").into_bytes())
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(map) as i64,
        Part::Part2 => part2(map) as i64,
    }
}
