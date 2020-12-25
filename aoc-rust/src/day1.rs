use std::collections::HashSet;

fn parse() -> HashSet<i32> {
    let res : Result<HashSet<i32>, _> = std::fs::read_to_string("../input/day1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse())
        .collect();
    res.unwrap()      
}

fn part1() {
    let set = parse();
    println!("Day 1.a: {}",
        set.iter()
            .find_map(|x| set.get(&(2020-x)).map(|y| y*x))
            .unwrap());
}

fn part2() {
    let set = parse();
    println!("Day 1.b: {}",
        set.iter().enumerate()
            .find_map(|(ndx,x)| {
                set.iter()
                    .skip(ndx+1)
                    .find_map(|y| set.get(&(2020-y-x)).map(|z| x*y*z) )
            }).unwrap());
}

pub fn run() {
    part1();
    part2();
}