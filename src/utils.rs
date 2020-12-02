use std::{fmt::Debug, str::FromStr};

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
