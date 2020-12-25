fn parse() -> Vec<Vec<usize>> {
    std::fs::read_to_string("../input/day3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().map(|c| (c == '#') as usize).collect()
        })
        .collect()
}

fn part1() {
    println!("Day 3.a: {}", parse().iter()
        .zip((0 as usize..).step_by(3))
        .map(|(v,x)| v[x%v.len()] as usize)
        .sum::<usize>());
}


fn part2() {
    println!("Day 3.b: {}", parse().iter()
        .enumerate()
        .fold(vec![0,0,0,0,0], |mut s,(y,v)| {
            s[0] += v[y%v.len()];
            s[1] += v[(y*3)%v.len()];
            s[2] += v[(y*5)%v.len()];
            s[3] += v[(y*7)%v.len()];
            
            if (y % 2) == 0 {
                s[4] += v[(y/2) % v.len()];
            }
            s
        }).iter().product::<usize>());
}

pub fn run() {
    part1();
    part2();
}