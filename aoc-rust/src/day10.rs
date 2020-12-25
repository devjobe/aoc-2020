fn parse() -> Vec<i32> {
    std::fs::read_to_string("../input/day10.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1() {
    let mut data = parse();
    data.push(0);
    data.sort();

    let res = data
        .iter()
        .skip(1)
        .zip(data.iter())
        .map(|(a, b)| a - b)
        .fold((0, 1), |acc, x| match x {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => unreachable!(),
        });
    println!("Day 10.a: {}", res.0 * res.1);
}

fn get_connections(data: &Vec<i32>, conn: &Vec<usize>, value: i32) -> usize {
    if let Ok(ndx) = data.binary_search(&value) {
        conn[ndx]
    } else {
        0
    }
}

fn part2() {
    let mut data = parse();
    data.push(0);
    data.sort();

    let mut connections: Vec<usize> = vec![1; data.len()];

    for index in (0..data.len() - 1).rev() {
        let value = data[index];
        connections[index] = get_connections(&data, &connections, value + 1)
            + get_connections(&data, &connections, value + 2)
            + get_connections(&data, &connections, value + 3);
    }
    println!("Day 10.b: {}", connections[0]);
}

pub fn run() {
    part1();
    part2();
}
