use std::collections::HashMap;
use std::collections::HashSet;

fn parse_coords(mut line: &str) -> (i32,i32) {

    let (mut x, mut y) : (i32,i32) = (0,0);

    while line.len() > 0 {
        if line.starts_with("se") {
            line = &line[2..];
            x += 1;
            y -= 1;
        }
        else if line.starts_with("nw") {
            line = &line[2..];
            x -= 1;
            y += 1;
        }
        else if line.starts_with("sw") {
            line = &line[2..];
            y -= 1;
        }
        else if line.starts_with("ne") {
            line = &line[2..];
            y += 1;
        }
        else if line.starts_with("w") {
            line = &line[1..];
            x -= 1;

        }
        else if line.starts_with("e") {
            line = &line[1..];
            x += 1;
        } else  {
            unreachable!();
        }
    }
    (x,y)    
}

pub fn run() {
    let input = std::fs::read_to_string("../input/day24.txt").unwrap();
    let mut tiles = HashSet::new();

    
    for line in input.lines() {
        let (x,y) = parse_coords(line);

        if tiles.contains(&(x,y)) {
            tiles.remove(&(x,y));
        } else {
            tiles.insert((x,y));
        }
    }

    println!("Day 24.a: {}", tiles.len());

    for _ in 0..100 {
        let mut whites = HashMap::new();


        let mut result = HashSet::new();

        result.reserve(5000);

        for (x,y) in &tiles {
            let (x,y) = (*x,*y);

            let mut black = 0;
            let mut check_tile = |x,y| {
                if tiles.contains(&(x,y)) {
                    black += 1;
                } else {
                    whites.entry((x,y)).and_modify(|x| *x += 1).or_insert(1);
                }
            };

            check_tile(x-1,y);
            check_tile(x+1,y);
            check_tile(x,y-1);
            check_tile(x,y+1);
            check_tile(x-1,y+1);
            check_tile(x+1,y-1);

            if black == 1 || black == 2 {
                result.insert((x,y));
            }        
        }

        for ((x,y), count) in whites {
            if count == 2 {
                result.insert((x,y));
            }
        }


        tiles = result;

    }

    println!("Day 24.b: {}", tiles.len());

}