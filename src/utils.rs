use std::{fmt::Debug, fs::File, io::BufRead, io::BufReader, str::FromStr};

pub fn count_if<P, T>(entries: &[T], p: P) -> usize
where
    P: Fn(&T) -> bool,
{
    entries.iter().filter(|e| p(*e)).count()
}

pub fn split_trim(s: &str, c: char) -> Vec<&str> {
    s.split(c).map(|s| s.trim()).collect::<Vec<&str>>()
}

pub fn parse_seq<'a, I: Iterator<Item = U>, T: FromStr, U: ToString>(it: I) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    it.map(|s| {
        s.to_string()
            .trim()
            .parse()
            .expect("failed to parse element")
    })
    .collect::<Vec<T>>()
}

pub fn parse_lines<T: FromStr>(reader: BufReader<File>) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    parse_seq(reader.lines().map(|s| s.expect("failed to read line")))
}
