use std::collections::HashMap;

fn parse() -> String {
    std::fs::read_to_string("../input/day14.txt")
        .unwrap()
}


fn part1() { 
    let mut andm = 0i64;
    let mut orm = 0i64;
    let mut mem : HashMap<i64,i64> = HashMap::new();
    
    for ins in parse().lines() {
        if ins.starts_with("mask = ") {
            andm = 0;
            orm = 0;
            for ch in ins[7..].chars() {
                andm = andm << 1;
                orm = orm << 1;

                if ch == '1' {
                    orm |= 1;
                } else if ch == 'X' {
                    andm |= 1;
                }
            }
        } else {
            let (k,b) : (i64,i64);
            scan!(ins.bytes() => "mem[{}] = {}", k,b);
            
            let v = (b & andm) | orm;

            mem.insert(k,v);
        }
    }


    println!("Day 14.a: {}", mem.values().sum::<i64>());
}



fn part2() { 
    let mut andm = 0i64;
    let mut orm = 0i64;
    let mut mem : HashMap<i64,i64> = HashMap::new();
    let mut floating : Vec<i64> = Vec::new();


    for ins in parse().lines() {
        if ins.starts_with("mask = ") {
            andm = 0;
            orm = 0;
            let mut fl = 0i64;
            floating.clear();
            for ch in ins[7..].chars() {
                andm <<= 1;
                orm <<= 1;
                fl += 1;

                if ch == '1' {
                    orm |= 1;
                } else if ch == '0' {
                    andm |= 1;
                } else {
                    floating.push(fl);
                }
            }

            for f in &mut floating {
                *f = 1 << (fl-*f);
            }
        } else {
            let (a,v) : (i64,i64);
            scan!(ins.bytes() => "mem[{}] = {}", a,v);
            
            let base = (a & andm) | orm;


            
            for n in 0..(1 << floating.len()) as i64 {
                let mut k = base;
                for (ndx, f) in floating.iter().enumerate() {
                    if (n & (1 << ndx)) != 0 {
                        k |= *f;
                    }
                }
                mem.insert(k,v);
            }
        }
    }


    println!("Day 14.b: {}", mem.values().sum::<i64>());
}

pub fn run() {
    part1();
    part2();
}