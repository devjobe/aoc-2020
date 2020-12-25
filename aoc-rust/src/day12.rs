fn parse() -> Vec<(char,i32)> {
    std::fs::read_to_string("../input/day12.txt")
        .unwrap()
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse::<i32>().unwrap()))
        .collect()
}

fn part1() {

    let mut direction = 0;
    let mut x = 0;
    let mut y = 0;
    for (c,n) in parse() {
        match c {
            'F' => {
                match direction {
                    0 => x += n,
                    1 => y -= n,
                    2 => x -= n,
                    3 => y += n,
                    _ => unreachable!()
                }
            },
            'L' => {
                direction = (direction - n / 90).rem_euclid(4);
            },
            'R' => {
                direction = (direction + n / 90).rem_euclid(4);
            },
            'E' => { x += n; },
            'S' => { y -= n; },
            'W' => { x -= n; },
            'N' => { y += n; },
            
            _ => unreachable!(),
        }
    }
    println!("Day 12.a: {}", x.abs()+y.abs());
}


fn part2() {

    let mut x = 10;
    let mut y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;

    for (c,n) in parse() {
        match c {
            'F' => {
                ship_x += x*n;
                ship_y += y*n;
            },
            'L' => {
                for _ in 0..(n/90) {
                    std::mem::swap(&mut x, &mut y);
                    x = -x;
                }
            },
            'R' => {
                for _ in 0..(n/90) {
                    std::mem::swap(&mut x, &mut y);
                    y = -y;
                }
            },
            'E' => { x += n; },
            'S' => { y -= n; },
            'W' => { x -= n; },
            'N' => { y += n; },
            
            _ => unreachable!(),
        }
    }
    println!("Day 12.b: {}", ship_x.abs()+ship_y.abs());
}

pub fn run() {
    part1();
    part2();
}