fn parse() -> Vec<i64> {
    std::fs::read_to_string("../input/day9.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn is_sum_of_any_pair(n: i64, list: &[i64]) -> bool {
    for (index, i) in list.iter().enumerate() {
        for j in list.iter().skip(index + 1) {
            if i + j == n {
                return true;
            }
        }
    }
    false
}

fn find_non_sum_of_two(numbers: &Vec<i64>) -> i64 {
    for (index, &value) in numbers.iter().enumerate().skip(25) {
        if !is_sum_of_any_pair(value, &numbers[index - 25..index]) {
            return value;
        }
    }
    0
}

fn part1() {
    println!("Day 9.a: {}", find_non_sum_of_two(&parse()));
}

fn part2() {
    let data = parse();

    let target = find_non_sum_of_two(&data);

    for index in 0..data.len() {
        let mut sum: i64 = data[index];
        for (end, n) in data.iter().enumerate().skip(index + 1) {
            sum += n;
            if sum == target {
                let mut v = Vec::from(&data[index..=end]);
                v.sort();
                println!("Day 9.b: {}", v.first().unwrap() + v.last().unwrap());
                return;
            }
        }
    }
}

pub fn run() {
    part1();
    part2();
}
