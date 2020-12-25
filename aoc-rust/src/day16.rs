use std::ops::RangeInclusive;

fn parse() -> (
    Vec<(String, RangeInclusive<i32>, RangeInclusive<i32>)>,
    Vec<i32>,
    Vec<Vec<i32>>,
) {
    let input = std::fs::read_to_string("../input/day16.txt").unwrap();
    let mut lines = input.lines();

    let map = (&mut lines)
        .take_while(|x| x.len() != 0)
        .map(|line| {
            let ndx = line.find(": ").unwrap();
            let key = &line[..ndx];
            let value = &line[ndx + 2..];
            let (a, b, c, d): (i32, i32, i32, i32);
            scan!(value.bytes() => "{}-{} or {}-{}", a,b,c,d);
            (key.to_string(), (a..=b), (c..=d))
        })
        .collect();

    let ticket = (&mut lines)
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let tickets = (&mut lines)
        .skip(2)
        .map(|line| line.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    (map, ticket, tickets)
}

fn part1() {
    let (ranges, _, tickets) = parse();

    let mut error_rate = 0;
    for t in &tickets {
        for n in t {
            if ranges
                .iter()
                .all(|(_, r1, r2)| !r1.contains(n) && !r2.contains(n))
            {
                error_rate += n;
            }
        }
    }

    println!("Day 16.a: {}", error_rate);
}

fn is_valid_ticket(
    t: &Vec<i32>,
    ranges: &Vec<(String, RangeInclusive<i32>, RangeInclusive<i32>)>,
) -> bool {
    for n in t {
        if ranges
            .iter()
            .all(|(_, r1, r2)| !r1.contains(n) && !r2.contains(n))
        {
            return false;
        }
    }
    true
}

fn all_fields_in_ranges(
    valid_tickets: &Vec<&Vec<i32>>,
    field: usize,
    r1: &RangeInclusive<i32>,
    r2: &RangeInclusive<i32>,
) -> bool {
    for t in valid_tickets {
        let n = t[field];
        if !r1.contains(&n) && !r2.contains(&n) {
            return false;
        }
    }
    true
}

fn part2() {
    let (ranges, ticket, tickets) = parse();

    let valid_tickets = tickets
        .iter()
        .filter(|t| is_valid_ticket(t, &ranges))
        .collect::<Vec<_>>();

    let mut names = vec![false; ticket.len()];
    let mut set = vec![false; ranges.len()];

    let mut process = true;
    let mut res = 1i64;
    while process {
        process = false;

        for (field, name) in names.iter_mut().enumerate() {
            if !*name {
                let n = ticket[field];
                let mut f = None;
                for (idx, (_, r1, r2)) in ranges.iter().enumerate() {
                    if !set[idx]
                        && (r1.contains(&n) || r2.contains(&n))
                        && all_fields_in_ranges(&valid_tickets, field, r1, r2)
                    {
                        if f.is_none() {
                            f = Some(idx);
                        } else {
                            f = None;
                            break;
                        }
                    }
                }

                if let Some(idx) = f {
                    set[idx] = true;
                    *name = true;
                    process = true;

                    if ranges[idx].0.starts_with("departure ") {
                        res *= n as i64;
                    }
                }
            }
        }
    }
    println!("Day 16.b: {}", res);
}
pub fn run() {
    part1();
    part2();
}
