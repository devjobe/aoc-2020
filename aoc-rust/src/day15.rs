use std::collections::HashMap;

fn find_nth(input: &[usize], nth: usize) -> usize {
    let mut game: HashMap<usize, usize> = HashMap::new();
    for (i, &n) in input.iter().enumerate() {
        game.insert(n, i + 1);
    }

    let mut last = *input.last().unwrap();
    for round in input.len()..nth {
        let next = match game.get(&last) {
            Some(&n) => round - n,
            None => 0,
        };
        game.insert(last, round);
        last = next;
    }

    last
}

fn part1(input: &[usize]) {
    println!("Day 15.a: {}", find_nth(input, 2020));
}

fn part2(input: &[usize]) {
    println!("Day 15.b: {}", find_nth(input, 30000000));
}

pub fn run() {
    let input = &[1, 2, 16, 19, 18, 0];
    part1(input);
    part2(input);
}
