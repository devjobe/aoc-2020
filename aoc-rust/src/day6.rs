use itertools::Itertools;
use std::collections::HashSet;

fn part1() {
    println!(
        "Day 6.a: {}",
        std::fs::read_to_string("../input/day6.txt")
            .unwrap()
            .split("\r\n\r\n")
            .map(|s| {
                s.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<HashSet<char>>()
                    .len()
            })
            .sum::<usize>()
    );
}

fn part2() {
    println!(
        "Day 6.b: {}",
        std::fs::read_to_string("../input/day6.txt")
            .unwrap()
            .split("\r\n\r\n")
            .map(|s| {
                s.lines()
                    .map(|x| x.chars().collect::<HashSet<char>>())
                    .fold1(|acc, set| acc.intersection(&set).copied().collect::<HashSet<char>>())
                    .map(|set| set.len())
                    .unwrap_or_default()
            })
            .sum::<usize>()
    );
}

pub fn run() {
    part1();
    part2();
}
