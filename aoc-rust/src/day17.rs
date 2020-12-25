use std::collections::HashMap;
use std::collections::HashSet;

fn parse() -> HashSet<(i32, i32, i32)> {
    std::fs::read_to_string("../input/day17.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '#' => Some((x as i32, y as i32, 0)),
                    _ => None,
                })
        })
        .flatten()
        .collect()
}

fn active_neighbours(
    &(x, y, z): &(i32, i32, i32),
    pd: &HashSet<(i32, i32, i32)>,
    inactive: &mut HashMap<(i32, i32, i32), usize>,
) -> usize {
    let mut active = 0;

    for &a in [x - 1, x, x + 1].iter() {
        for &b in [y - 1, y, y + 1].iter() {
            for &c in [z - 1, z, z + 1].iter() {
                let cube = (a, b, c);
                if pd.contains(&cube) {
                    active += 1;
                } else {
                    inactive.entry(cube).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }
    }

    active - 1
}

fn part1() {
    let mut pd = parse();

    for _ in 0..6 {
        let mut cycled = HashSet::new();

        let mut inactive: HashMap<(i32, i32, i32), usize> = HashMap::new();
        for cube in &pd {
            if (2..=3).contains(&active_neighbours(cube, &pd, &mut inactive)) {
                cycled.insert(*cube);
            }
        }

        for (cube, _) in inactive.drain().filter(|(_, count)| *count == 3) {
            cycled.insert(cube);
        }

        pd = cycled;
    }

    println!("Day 17.a: {}", pd.len());
}

fn parse2() -> HashSet<(i32, i32, i32, i32)> {
    std::fs::read_to_string("../input/day17.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '#' => Some((x as i32, y as i32, 0, 0)),
                    _ => None,
                })
        })
        .flatten()
        .collect()
}

fn active_neighbours2(
    &(x, y, z, w): &(i32, i32, i32, i32),
    pd: &HashSet<(i32, i32, i32, i32)>,
    inactive: &mut HashMap<(i32, i32, i32, i32), usize>,
) -> usize {
    let mut active = 0;

    for &a in [x - 1, x, x + 1].iter() {
        for &b in [y - 1, y, y + 1].iter() {
            for &c in [z - 1, z, z + 1].iter() {
                for &d in [w - 1, w, w + 1].iter() {
                    let hypercube = (a, b, c, d);
                    if pd.contains(&hypercube) {
                        active += 1;
                    } else {
                        inactive
                            .entry(hypercube)
                            .and_modify(|x| *x += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }

    active - 1
}

fn part2() {
    let mut pd = parse2();

    for _ in 0..6 {
        let mut cycled = HashSet::new();

        let mut inactive: HashMap<(i32, i32, i32, i32), usize> = HashMap::new();
        for cube in &pd {
            if (2..=3).contains(&active_neighbours2(cube, &pd, &mut inactive)) {
                cycled.insert(*cube);
            }
        }

        for (cube, _) in inactive.drain().filter(|(_, count)| *count == 3) {
            cycled.insert(cube);
        }

        pd = cycled;
    }

    println!("Day 17.b: {}", pd.len());
}

pub fn run() {
    part1();
    part2();
}
