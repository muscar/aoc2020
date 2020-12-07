use crate::part::Part;

use std::io::{BufRead, BufReader};
use std::{collections::BTreeMap, collections::HashSet, fs::File};

fn parse_contents(contents: &str) -> Vec<(i64, String)> {
    let cs = contents.split(", ").map(|s| {
        let s = s.replace(" bags", "").replace(" bag", "");
        let idx = s.find(char::is_whitespace).unwrap();
        let (cnt, colour) = s.split_at(idx);
        (cnt.parse().unwrap(), colour.trim().to_string())
    });
    cs.collect()
}

fn parse_rule(rule: &str) -> (String, Vec<(i64, String)>) {
    let mut parts = rule[..rule.len() - 1].split("contain").map(|s| s.trim());
    let container = parts
        .next()
        .map(|s| s.replace(" bags", "").replace(" bag", ""));
    match (container, parts.next()) {
        (Some(container), Some("no other bags")) => (container, Vec::new()),
        (Some(container), Some(contents)) => (container, parse_contents(contents)),
        _ => panic!("failed to parse rule"),
    }
}

fn part1(rules: &[String], colour: &str) -> i64 {
    let mut adj = BTreeMap::new();
    for r in rules {
        let (container, contents) = parse_rule(r);
        for (_, colour) in contents {
            if !adj.contains_key(&colour) {
                adj.insert(colour.clone(), Vec::new());
            }
            adj.get_mut(&colour).unwrap().push(container.clone())
        }
    }
    let mut cnt = 0;
    let mut fringe = adj.get(colour).unwrap().clone();
    let mut seen = HashSet::new();
    while !fringe.is_empty() {
        let curr = fringe.remove(0);
        if seen.insert(curr.clone()) {
            cnt += 1;
            if let Some(es) = adj.get(&curr) {
                fringe.extend(es.clone());
            }
        }
    }
    cnt
}

fn count_bags(adj: &BTreeMap<String, Vec<(i64, String)>>, colour: &str) -> i64 {
    if !adj.contains_key(colour) {
        return 0;
    }
    adj.get(colour)
        .unwrap()
        .iter()
        .map(|(cnt, colour)| cnt + cnt * count_bags(adj, colour))
        .sum()
}

fn part2(rules: &[String], colour: &str) -> i64 {
    let mut adj = BTreeMap::new();
    for r in rules {
        let (container, contents) = parse_rule(r);
        for (cnt, colour) in contents {
            if !adj.contains_key(&container) {
                adj.insert(container.clone(), Vec::new());
            }
            adj.get_mut(&container).unwrap().push((cnt, colour.clone()))
        }
    }
    count_bags(&adj, colour)
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let rules = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&rules, "shiny gold"),
        Part::Part2 => part2(&rules, "shiny gold"),
    }
}
