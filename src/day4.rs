use crate::part::Part;

use std::fs::File;
use std::{
    collections::BTreeMap,
    error::Error,
    io::{BufRead, BufReader, Lines},
    ops::RangeInclusive,
    path::Path,
};

struct Entries {
    lines: Lines<BufReader<File>>,
    curr: BTreeMap<String, String>,
}

impl Entries {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let f = File::open(path).expect("failed to open input file");
        let reader = BufReader::new(f);
        Ok(Self {
            lines: reader.lines(),
            curr: BTreeMap::new(),
        })
    }
}

impl Iterator for Entries {
    type Item = BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.clear();
        while let Some(l) = self.lines.next() {
            match l.unwrap() {
                s if s.is_empty() => return Some(self.curr.clone()),
                s => {
                    for kvp in s.split(' ') {
                        let mut parts = kvp.split(':').map(|p| p.trim());
                        match (parts.next(), parts.next()) {
                            (Some(key), Some(value)) => {
                                self.curr.insert(key.to_string(), value.to_string());
                            }
                            _ => panic!("failed to parse key-value pair"),
                        }
                    }
                }
            }
        }
        if !self.curr.is_empty() {
            Some(self.curr.clone())
        } else {
            None
        }
    }
}

fn has_required_fields(fields: &BTreeMap<String, String>) -> bool {
    let required_fields = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    required_fields.iter().all(|&f| fields.contains_key(f))
}

fn validate_range(s: &str, r: RangeInclusive<i64>) -> bool {
    s.parse::<i64>().map(|n| r.contains(&n)).unwrap_or_default()
}

fn validate_field(name: &str, value: &str) -> bool {
    match name {
        "byr" => validate_range(value, 1920..=2002),
        "iyr" => validate_range(value, 2010..=2020),
        "eyr" => validate_range(value, 2020..=2030),
        "hgt" if value.ends_with("cm") => validate_range(&value[..value.len() - 2], 150..=193),
        "hgt" if value.ends_with("in") => validate_range(&value[..value.len() - 2], 59..=76),
        "hcl" if value.starts_with('#') && value.len() == 7 => {
            value[1..].chars().all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => {
            value == "amb"
                || value == "blu"
                || value == "brn"
                || value == "gry"
                || value == "grn"
                || value == "hzl"
                || value == "oth"
        }
        "pid" if value.len() == 9 => value.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => false,
    }
}

fn is_valid_pass(pass: &BTreeMap<String, String>) -> bool {
    has_required_fields(pass) && pass.iter().all(|(k, v)| validate_field(&k, &v))
}

fn part1<It: Iterator<Item = BTreeMap<String, String>>>(entries: It) -> usize {
    entries.filter(|e| has_required_fields(e)).count()
}

fn part2<It: Iterator<Item = BTreeMap<String, String>>>(entries: It) -> usize {
    entries.filter(|e| is_valid_pass(e)).count()
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let es = Entries::from_file(input_path).expect("failed to open input file");
    match part {
        Part::Part1 => part1(es) as i64,
        Part::Part2 => part2(es) as i64,
    }
}
