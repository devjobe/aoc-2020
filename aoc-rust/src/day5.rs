fn seat_id(s: &str) -> i32 {
    s.chars().fold(0, |acc, x| {
        (acc << 1)
            | match x {
                'B' => 1,
                'R' => 1,
                _ => 0,
            }
    })
}

fn parse() -> Vec<i32> {
    std::fs::read_to_string("../input/day5.txt")
        .unwrap()
        .lines()
        .map(|line| seat_id(line))
        .collect()
}

fn part1() {
    println!("Day 5.a: {}", parse().iter().max().unwrap());
}


fn part2() {
    let mut s = parse();
    s.sort();
    println!(
        "Day 5.b: {}", s.iter()
        .skip(1)
        .zip(s.iter())
        .find_map(|(a, b)| match a - b {
            2 => Some(b + 1),
            _ => None,
        })
        .unwrap()    
    );
}

pub fn run() {
    part1();
    part2();
}
