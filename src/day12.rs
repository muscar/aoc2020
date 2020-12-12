use crate::part::Part;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

#[derive(Debug)]
enum Action {
    Move(Direction, i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

const DELTAS: [(i64, i64); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

fn go((x, y): (i64, i64), dir: usize, dist: i64) -> (i64, i64) {
    let (dx, dy) = DELTAS[dir];
    (x + dx * dist, y + dy * dist)
}

fn part1(actions: &[Action]) -> usize {
    let (_, (x, y)) = actions.iter().fold((0, (0, 0)), |(d, coords), a| match a {
        Action::Move(dir, dist) => (d, go(coords, *dir as usize, *dist)),
        Action::Left(degrees) => {
            let mut x = d - degrees / 90;
            if x < 0 {
                x += DELTAS.len() as i64;
            }
            (x, coords)
        }
        Action::Right(degrees) => ((d + (degrees / 90)) % 4, coords),
        Action::Forward(dist) => (d, go(coords, d as usize, *dist)),
    });

    (x.abs() + y.abs()) as usize
}

fn rotate((x, y): (i64, i64), degrees: i64) -> (i64, i64) {
    let (sin90, cos90) = DELTAS[(degrees.abs() / 90 - 1) as usize];
    if degrees < 0 {
        // counterclockwise
        // x' = x * cos(d) - y * sin(d);
        // y' = x * sin(d) + y * cos(d);
        (x * cos90 - y * sin90, x * sin90 + y * cos90)
    } else {
        // clockwise
        // x' = x * cos(d) + y * sin(d);
        // y' = -x * sin(d) + y * cos(d);
        (x * cos90 + y * sin90, -x * sin90 + y * cos90)
    }
}

fn part2(actions: &[Action]) -> usize {
    let (_, (x, y)) = actions
        .iter()
        .fold(((10, 1), (0, 0)), |(waypoint, coords), a| match a {
            Action::Move(dir, dist) => (go(waypoint, *dir as usize, *dist), coords),
            Action::Left(degrees) => (rotate(waypoint, -*degrees), coords),
            Action::Right(degrees) => (rotate(waypoint, *degrees), coords),
            Action::Forward(dist) => (
                waypoint,
                (coords.0 + waypoint.0 * dist, coords.1 + waypoint.1 * dist),
            ),
        });
    (x.abs() + y.abs()) as usize
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let actions = reader
        .lines()
        .map(|l| match l.unwrap().split_at(1) {
            ("N", v) => Action::Move(Direction::North, v.parse().unwrap()),
            ("S", v) => Action::Move(Direction::South, v.parse().unwrap()),
            ("E", v) => Action::Move(Direction::East, v.parse().unwrap()),
            ("W", v) => Action::Move(Direction::West, v.parse().unwrap()),
            ("L", v) => Action::Left(v.parse().unwrap()),
            ("R", v) => Action::Right(v.parse().unwrap()),
            ("F", v) => Action::Forward(v.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&actions) as i64,
        Part::Part2 => part2(&actions) as i64,
    }
}
