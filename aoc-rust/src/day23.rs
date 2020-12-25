use std::collections::HashMap;

fn eval_game(state: &mut HashMap<usize, usize>, start: usize, rounds: usize) {

    let total = state.len();
    let mut current = start;

    for _ in 0..rounds {

        let a = *state.get(&current).unwrap();
        let b = *state.get(&a).unwrap();
        let c = *state.get(&b).unwrap();

        let mut destination = (current + total-2) % total + 1;


        while destination == a || destination == b || destination == c {
            destination = (destination + total-2) % total + 1;
        }

        *state.get_mut(&current).unwrap() = *state.get(&c).unwrap();
        *state.get_mut(&c).unwrap() = *state.get(&destination).unwrap();
        *state.get_mut(&destination).unwrap() = a;

        current = *state.get_mut(&current).unwrap();
    }    
}

fn part1() {
    let data = [2,1,9,3,4,7,8,6,5];

    let mut state : HashMap<usize,usize> = HashMap::new();
    state.extend(data.iter().copied().zip(data.iter().copied().cycle().skip(1)));
    
    eval_game(&mut state, *data.first().unwrap(), 100);

    print!("Day 23.a: ");
    let mut n = 1;
    loop {
        n = *state.get(&n).unwrap();
        if n == 1 {
            break;
        }

        print!("{}", n);
    }
    println!();
}

fn part2()
{    
    let data = [2,1,9,3,4,7,8,6,5];

    let mut state : HashMap<usize,usize> = HashMap::new();

    state.reserve(1000000);
    
    state.extend(data.iter().copied().zip(data.iter().copied().cycle().skip(1)));
    state.extend((10..=1000000).zip(11..=1000001));
    *state.get_mut(data.last().unwrap()).unwrap() = 10;
    *state.get_mut(&1000000).unwrap() = *data.first().unwrap();
    
    eval_game(&mut state, *data.first().unwrap(), 10000000);

    let a = *state.get(&1).unwrap() as u64;
    let b = *state.get(&(a as usize)).unwrap() as u64;

    println!("Day 23.b: {}", a * b);
}

pub fn run() {
    part1();
    part2();
}