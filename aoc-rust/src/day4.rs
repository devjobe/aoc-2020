use std::{collections::HashMap, ops::RangeInclusive};

fn parse() -> Vec<Vec<(String, String)>> {
    std::fs::read_to_string("../input/day4.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|values| {
            values
                .split_whitespace()
                .map(|kw| {
                    let (a, b): (String, String);
                    scan!(kw.bytes() => "{}:{}", a,b);
                    (a, b)
                })
                .collect()
        })
        .collect()
}

fn part1() {
    let valid_keys: HashMap<&'static str, i32> = [
        ("cid", 0),
        ("ecl", 1),
        ("pid", 2),
        ("eyr", 3),
        ("hcl", 4),
        ("byr", 5),
        ("iyr", 6),
        ("hgt", 7),
    ]
    .iter()
    .cloned()
    .collect();

    println!(
        "Day 4.a: {}",
        parse()
            .iter()
            .filter(|values| {
                let res = values
                    .iter()
                    .fold(0, |acc, (k, _)| match valid_keys.get(k.as_str()) {
                        Some(ndx) => acc | (1 << ndx),
                        _ => -1,
                    });
                res != -1 && (res & 254) == 254
            })
            .count()
    );
}

fn part2() {
    fn in_range(s: &str, r: RangeInclusive<i32>) -> bool {
        s.parse::<i32>().map(|x| r.contains(&x)) == Ok(true)
    }

    let valid_keys: HashMap<&str, (i32, fn(&str) -> bool)> = [
        ("cid", (0, (|_: &str| true) as fn(&str) -> bool)),
        (
            "ecl",
            (1, |x| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x)
            }),
        ),
        (
            "pid",
            (2, |x| {
                x.len() == 9 && x.matches(|x: char| x.is_digit(10)).count() == 9
            }),
        ),
        ("eyr", (3, |x| in_range(x, 2020..=2030))),
        (
            "hcl",
            (4, |s: &str| {
                s.len() == 7
                    && &s[..1] == "#"
                    && s.matches(|x: char| x.is_digit(10) || x.is_ascii_lowercase())
                        .count()
                        == 6
            }),
        ),
        ("byr", (5, |x| in_range(x, 1920..=2002))),
        ("iyr", (6, |x| in_range(x, 2010..=2020))),
        (
            "hgt",
            (7, |x| {
                (x.ends_with("cm") && in_range(&x[..x.len() - 2], 150..=193))
                    || (x.ends_with("in") && in_range(&x[..x.len() - 2], 59..=76))
            }),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    println!(
        "Day 4.b: {:?}",
        parse()
            .iter()
            .filter(|values| {
                let res = values
                    .iter()
                    .fold(0, |acc, (k, w)| match valid_keys.get(k.as_str()) {
                        Some((ndx, validate)) if validate(w) => acc | (1 << ndx),
                        _ => -1,
                    });
                res != -1 && (res & 254 == 254)
            })
            .count()
    );
}

pub fn run() {
    part1();
    part2();
}
