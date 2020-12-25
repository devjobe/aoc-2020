use std::collections::HashSet;

fn split_once<'a>(input: &'a str, pat: &str) -> (&'a str, &'a str) {
    match input.find(pat) {
        Some(index) => (&input[..index], &input[index + pat.len()..]),
        None => (input, ""),
    }
}

fn calc_score(cards: &Vec<usize>) -> i64 {
    cards.iter().zip((1..=cards.len()).rev()).map(|(x,y)| (*x as i64) * (y as i64)).sum::<i64>()
}

fn part1(mut p1: Vec<usize>, mut p2: Vec<usize>) {    
    let score = loop {
        let c1 = p1.remove(0);
        let c2 = p2.remove(0);
                
        if c1 > c2 {
            p1.push(c1);
            p1.push(c2);

            if p2.len() == 0 {
                break calc_score(&p1);
            }
        } else {
            p2.push(c2);
            p2.push(c1);
            if p1.len() == 0 {
                break calc_score(&p2);
            }
        }
    };

    println!("Day 22.a: {}", score);
}


enum Winner {
    Player1,
    Player2,
}

fn recurse_play(p1: &mut Vec<usize>, p2: &mut Vec<usize>) -> Winner {
    let mut previous_hands : HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    loop {
        let hands = (p1.clone(), p2.clone());
        if previous_hands.contains(&hands) {
            return Winner::Player1;
        } else {
            previous_hands.insert(hands);
        }

        let c1 = p1.remove(0);
        let c2 = p2.remove(0);

        let winner = if c1 <= p1.len() && c2 <= p2.len() {
            recurse_play(&mut p1[0..c1].iter().copied().collect(), &mut p2[0..c2].iter().copied().collect())
        } else if c1 > c2 {
            Winner::Player1
        } else {
            Winner::Player2
        };
                
        match winner {
            Winner::Player1 => {
                p1.push(c1);
                p1.push(c2);
    
                if p2.len() == 0 {
                    return Winner::Player1;
                }
            },
            Winner::Player2 => {
                p2.push(c2);
                p2.push(c1);
                if p1.len() == 0 {
                    return Winner::Player2;
                }
            }
        }
    };    
}

pub fn run() {
    let input = std::fs::read_to_string("../input/day22.txt").unwrap();

    let (p1,p2) = split_once(&input, "\r\n\r\n");
    let mut p1 = p1.lines().skip(1).map(|x| x.parse::<usize>().expect("Expected number.")).collect::<Vec<_>>();
    let mut p2 = p2.lines().skip(1).map(|x| x.parse::<usize>().expect("Expected number.")).collect::<Vec<_>>();
    
    part1(p1.clone(), p2.clone());

    let score = match recurse_play(&mut p1, &mut p2) {
        Winner::Player1 => calc_score(&p1),
        Winner::Player2 => calc_score(&p2)
    };

    println!("Day 22.b: {}", score);
}