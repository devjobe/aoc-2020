fn parse() -> (i32, Vec<i32>) {
    let input: Vec<String> = std::fs::read_to_string("../input/day13.txt")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    let n = input[0].parse::<i32>().unwrap();
    let busses = input[1]
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (n, busses)
}

fn part1() {
    let (time, busses) = parse();
    println!(
        "Day 13.a: {}",
        busses
            .iter()
            .map(|x| (x - (time % x), x))
            .min_by_key(|x| x.0)
            .map(|(x, y)| x * y)
            .unwrap()
    );
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(data: &[(i64, i64)]) -> Option<i64> {
    let prod = data.iter().map(|&(_, n)| n).product::<i64>();
    let mut sum = 0;
    for &(residue, modulus) in data {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p;
    }
    Some(sum % prod)
}

fn parse2() -> Vec<(i64, i64)> {
    let input: Vec<String> = std::fs::read_to_string("../input/day13.txt")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    input[1]
        .split(',')
        .map(|x| {
            if x != "x" {
                x.parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .enumerate()
        .filter_map(|(i, n)| {
            if n != 0 {
                Some(((n - (i as i64 % n)) % n, n))
            } else {
                None
            }
        })
        .collect()
}

fn part2() {
    println!("Day 13.b: {}", chinese_remainder(&parse2()).unwrap());
}

pub fn run() {
    part1();
    part2();
}
