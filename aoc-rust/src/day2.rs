fn parse() -> Vec<(usize,usize,char,String)> {    
    std::fs::read_to_string("../input/day2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (a,b,c,d) : (usize, usize, char, String);
            scan!(line.bytes() => "{}-{} {}: {}", a,b,c,d);
            (a,b,c,d)
        })
        .collect()
}

fn part1() {    
    println!("Day 2.a: {}",
        parse()
            .iter()
            .filter(|(min,max,ch,s)|{
                (*min..=*max).contains(&s.matches(*ch).count())
            })
            .count());
}

fn part2() {
    println!("Day 2.b: {}", 
        parse()
            .iter()
            .filter(|(min,max,ch,s)|{
                (s.chars().nth(*min-1) == Some(*ch)) !=
                (s.chars().nth(*max-1) == Some(*ch))
            })
            .count());
}

pub fn run() {
    part1();
    part2();
}