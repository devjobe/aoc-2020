fn parse() -> Box<[Box<[char]>]> {
    std::fs::read_to_string("../input/day11.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn adjacent_occupied(data: &Box<[Box<[char]>]>, row: usize, column: usize) -> usize {
    let mut occupied = 0;
    if row > 0 {
        let r = &data[row - 1];
        if column > 0 && r[column - 1] == '#' {
            occupied += 1;
        }

        if r[column] == '#' {
            occupied += 1;
        }

        match r.get(column + 1) {
            Some('#') => occupied += 1,
            _ => (),
        }
    }

    {
        let r = &data[row];
        if column > 0 && r[column - 1] == '#' {
            occupied += 1;
        }

        match r.get(column + 1) {
            Some('#') => occupied += 1,
            _ => (),
        }
    }

    if let Some(r) = data.get(row + 1) {
        if column > 0 && r[column - 1] == '#' {
            occupied += 1;
        }

        if r[column] == '#' {
            occupied += 1;
        }

        match r.get(column + 1) {
            Some('#') => occupied += 1,
            _ => (),
        }
    }

    occupied
}

fn part1() {
    let mut data = parse();

    loop {
        let mut changed = false;
        let mut new_data = data.clone();
        for (i, row) in new_data.iter_mut().enumerate() {
            for (j, seat) in row.iter_mut().enumerate() {
                if *seat == 'L' {
                    if adjacent_occupied(&data, i, j) == 0 {
                        changed = true;
                        *seat = '#';
                    }
                } else if *seat == '#' {
                    if adjacent_occupied(&data, i, j) >= 4 {
                        changed = true;
                        *seat = 'L';
                    }
                }
            }
        }

        if changed {
            data = new_data;
        } else {
            break;
        }
    }

    println!(
        "Day 11.a: {}",
        data.iter()
            .map(|row| { row.iter().filter(|x| **x == '#').count() })
            .sum::<usize>()
    );
}

fn is_occupied_nearest<'a, P>(rows: P, mut start: i32, direction: i32) -> usize
where
    P: Iterator<Item = &'a Box<[char]>>,
{
    for row in rows {
        start += direction;
        if start < 0 || start >= row.len() as i32 {
            break;
        }
        match row[start as usize] {
            'L' => break,
            '#' => return 1,
            _ => (),
        }
    }
    0
}

fn adjacent_occupied2(data: &Box<[Box<[char]>]>, row: usize, column: usize) -> usize {
    let is_occupied = |i, j| {
        if i > 0 {
            is_occupied_nearest(data[row + 1..].iter(), column as i32, j)
        } else if i < 0 {
            is_occupied_nearest(data[0..row].iter().rev(), column as i32, j)
        } else {
            if j < 0 {
                for c in data[row][0..column].iter().rev() {
                    match *c {
                        'L' => break,
                        '#' => return 1,
                        _ => (),
                    }
                }
            } else {
                for c in data[row][column+1..].iter() {
                    match *c {
                        'L' => break,
                        '#' => return 1,
                        _ => (),
                    }
                }
            }
            0
        }
    };
    is_occupied(-1, -1)
        + is_occupied(-1, 0) 
        + is_occupied(-1, 1)
        + is_occupied(0, 1)
        + is_occupied(1, 1)
        + is_occupied(1, 0)
        + is_occupied(1, -1)
        + is_occupied(0, -1)
        
}

fn part2() {
    let mut data = parse();

    loop {
        let mut changed = false;
        let mut new_data = data.clone();
        for (i, row) in new_data.iter_mut().enumerate() {
            for (j, seat) in row.iter_mut().enumerate() {
                
                if *seat == 'L' {
                    if adjacent_occupied2(&data, i, j) == 0 {
                        changed = true;
                        *seat = '#';
                    }
                } else if *seat == '#' {
                    if adjacent_occupied2(&data, i, j) >= 5 {
                        changed = true;
                        *seat = 'L';
                    }
                }
            }
        }

        if changed {
            data = new_data;
        } else {
            break;
        }
    }

    println!(
        "Day 11.b: {}",
        data.iter()
            .map(|row| { row.iter().filter(|x| **x == '#').count() })
            .sum::<usize>()
    );
}

pub fn run() {
    part1();
    part2();
}
