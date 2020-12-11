use crate::part::Part;

use std::io::{BufRead, BufReader};
use std::{fs::File, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        match (parts.next(), parts.next()) {
            (Some("nop"), Some(op)) => Ok(Instr::Nop(op.parse().unwrap())),
            (Some("acc"), Some(op)) => Ok(Instr::Acc(op.parse().unwrap())),
            (Some("jmp"), Some(op)) => Ok(Instr::Jmp(op.parse().unwrap())),
            _ => Err(format!("failed to parse instruction: {}", s)),
        }
    }
}

fn run_prog(prog: &[Instr]) -> Result<i64, i64> {
    let mut seen = vec![false; prog.len()];
    let mut ip = 0;
    let mut acc = 0;
    while ip < prog.len() && !seen[ip] {
        seen[ip] = true;
        match prog[ip] {
            Instr::Acc(n) => acc += n,
            Instr::Jmp(off) => {
                ip = (ip as i64 + off) as usize;
                continue;
            }
            Instr::Nop(_) => (),
        }
        ip += 1;
    }
    if ip >= prog.len() {
        return Ok(acc);
    }
    Err(acc)
}

fn part1(prog: &[Instr]) -> i64 {
    run_prog(prog).unwrap_or_else(|x| x)
}

fn part2(prog: &[Instr]) -> i64 {
    let mut off = 0;
    while let Some(idx) = prog
        .iter()
        .skip(off)
        .position(|i| matches!(i, Instr::Jmp(_) | Instr::Nop(_)))
    {
        let instr = match prog[off + idx] {
            Instr::Nop(x) => Instr::Jmp(x),
            Instr::Jmp(x) => Instr::Nop(x),
            i => i,
        };
        let patched = [&prog[..off + idx], &[instr], &prog[off + idx + 1..]].concat();
        if let Ok(x) = run_prog(&patched) {
            return x;
        }
        off = off + idx + 1;
    }
    0
}

pub fn run(part: Part, input_path: &str) -> i64 {
    let f = File::open(input_path).expect("failed to open input file");
    let reader = BufReader::new(f);
    let prog = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .map(|l| l.parse().expect("failed to parse entry"))
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => part1(&prog),
        Part::Part2 => part2(&prog),
    }
}
