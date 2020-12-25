#[derive(Clone, Copy)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse() -> Vec<(Ins, i32)> {
    std::fs::read_to_string("../input/day8.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let v = line[4..].parse::<i32>().unwrap();
            (
                match &line[..3] {
                    "acc" => Ins::Acc(v),
                    "jmp" => Ins::Jmp(v),
                    "nop" => Ins::Nop(v),
                    _ => unreachable!(),
                },
                0,
            )
        })
        .collect()
}

enum Eval {
    OutOfBounds(i32),
    Loop(i32),
    Exit(i32),
}

fn eval(ins: &mut Vec<(Ins, i32)>, gen: i32) -> Eval {
    let mut acc: i32 = 0;
    let mut pos: i32 = 0;
    loop {
        if pos as usize == ins.len() {
            return Eval::Exit(acc);
        } else if pos < 0 || pos as usize > ins.len() {
            return Eval::OutOfBounds(acc);
        }
        let mut ins = &mut ins[pos as usize];
        if ins.1 > gen as i32 {
            return Eval::Loop(acc);
        }
        ins.1 = gen as i32 + 1;

        match ins.0 {
            Ins::Acc(v) => {
                acc += v;
                pos += 1;
            }
            Ins::Jmp(off) => pos += off,
            Ins::Nop(_) => pos += 1,
        };
    }
}

fn part1() {
    println!(
        "Day 8.a: {}",
        match eval(&mut parse(), 0) {
            Eval::Loop(acc) => acc,
            _ => panic!("Not infinite loop."),
        }
    );
}

fn part2() {
    let mut ins = parse();
    for index in 0..ins.len() {
        let ip = ins[index];
        ins[index] = (
            match ip.0 {
                Ins::Acc(_) => continue,
                Ins::Jmp(v) => Ins::Nop(v),
                Ins::Nop(v) => Ins::Jmp(v),
            },
            0,
        );

        match eval(&mut ins, index as i32) {
            Eval::Exit(acc) => {
                println!("Day 8.b: {}", acc);
                break;
            }
            _ => (),
        }
        ins[index] = ip;
    }
}

pub fn run() {
    part1();
    part2();
}
