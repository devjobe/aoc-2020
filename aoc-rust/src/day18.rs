fn eval_int(expr: &str) -> (&str, u64) {
    let (a,b) = match expr.find(|x| x == ' ' || x == ')') {
        Some(index) => (&expr[index..], &expr[..index]),
        None => ("", expr),
    };
    (a, b.parse::<u64>().unwrap())
}

fn eval_term(expr: &str) -> (&str, u64) {
    let expr = expr.trim_start();

    if expr.starts_with('(') {
        let (expr, val) = eval(&expr[1..]);
        if !expr.starts_with(')') {
            panic!("Expected )");
        }
        (&expr[1..], val)
    } else {
        eval_int(expr)
    }
}

fn eval(expr: &str) -> (&str, u64) {
    let (mut expr, mut val) = eval_term(expr);

    if expr.len() == 0 {
        return ("", val);
    }

    loop {
        let mul = if expr.starts_with(" *") {
            true
        } else if expr.starts_with(" +") {
            false
        } else {
            return (expr, val);
        };

        let (e, val2) = eval_term(&expr[2..]);
        expr = e;
        val = if mul { val * val2 } else { val + val2 };
    }
}

fn part1(input: &str) {
    let mut sum = 0u64;
    for line in input.lines() {
        let (r, n) = eval(line);
        if r.len() > 0 {
            panic!("Expected empty return.");
        }
        sum += n;
    }

    println!("Day 18.a: {}", sum);
}

fn eval_term2(expr: &str) -> (&str, u64) {
    let expr = expr.trim_start();

    if expr.starts_with('(') {
        let (expr, val) = eval2(&expr[1..]);
        if !expr.starts_with(')') {
            panic!("Expected )");
        }
        (&expr[1..], val)
    } else {
        eval_int(expr)
    }
}


fn eval2(expr: &str) -> (&str, u64) {
    let (mut expr, mut val) = eval_term2(expr);

    if expr.len() == 0 {
        return ("", val);
    }

    loop {
        loop {
            if !expr.starts_with(" +") {
                break;
            };

            let (e, val2) = eval_term2(&expr[2..]);
            expr = e;
            val = val + val2;
        }

        if !expr.starts_with(" *") {
            return (expr, val);
        };

        let (e, val2) = eval2(&expr[2..]);
        expr = e;
        val = val * val2;
    }
}

fn part2(input: &str) {
    let mut sum = 0u64;
    for line in input.lines() {
        let (r, n) = eval2(line);
        if r.len() > 0 {
            panic!("Expected empty return.");
        }
        sum += n;
    }

    println!("Day 18.b: {}", sum);
}
pub fn run() {
    let input = std::fs::read_to_string("../input/day18.txt").unwrap();
    part1(&input);
    part2(&input);
}
