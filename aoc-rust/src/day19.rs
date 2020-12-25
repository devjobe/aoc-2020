use std::collections::HashMap;

fn split_once<'a>(input: &'a str, pat: &str) -> (&'a str, &'a str) {
    match input.find(pat) {
        Some(index) => (&input[..index], &input[index + pat.len()..]),
        None => (input, ""),
    }
}

enum Rule {
    Value(char),
    Sub(Vec<Vec<i32>>),
}


fn sub_matches<'a>(value: &'a str, rules: &HashMap<i32, Rule>, subrules: &Vec<i32>) -> Option<&'a str> {
    let mut v = value;
    for rule in subrules {
        if let Some(x) = matches(v, rules, *rule) {
            v = x;
        } else {
            return None;
        }
    }
    Some(v)
}

fn matches<'a>(value: &'a str, rules: &HashMap<i32, Rule>, id: i32) -> Option<&'a str> {
    let rule = rules.get(&id).unwrap();
    match rule {
        Rule::Value(ch) => if value.starts_with(*ch) { Some(&value[1..]) } else { None },
        Rule::Sub(multi) => {
            for subrules in multi {
                if let Some(v) = sub_matches(value, rules, subrules) {
                    return Some(v);
                }
            }
            None
        },
    }
}

fn parse_rules(input: &str) -> (HashMap<i32,Rule>, Vec<&str>) {
    let mut rules = HashMap::new();
    let mut values = Vec::new();
    for line in input.lines() {
        let (n, s) = split_once(line, ": ");

        if s.len() == 0 {
            values.push(n);
            continue;
        }

        let n = n.parse::<i32>().unwrap();
        if s == "\"a\"" {
            rules.insert(n, Rule::Value('a'));
        } else if s == "\"b\"" {
            rules.insert(n, Rule::Value('b'));
        } else {
            let multi = s
                .split('|')
                .map(|sub| {
                    sub.split_ascii_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();
            rules.insert(n, Rule::Sub(multi));
        }
    } 
    (rules, values)
}


fn loop_matches<'a>(value: &'a str, rules: &HashMap<i32, Rule>) -> Option<&'a str> {
    // 0: 8 11
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31

    let mut num42 = 0;

    let mut v = value;
    loop {
        if let Some(x) = matches(v, rules, 42) {
            v = x;
            num42 += 1;
        } else {
            break;
        }
    }

    if num42 < 2 {
        return None;
    }


    let mut num31 = 0;
    loop {
        if let Some(x) = matches(v, rules, 31) {
            v = x;
            num31 += 1;
        } else {
            break;
        }
    }

    if v != "" {
        return None;
    }

    if num31 < 1 || num42-1 < num31 {
        return None;
    }

    Some("")
}

pub fn run() {
    let input = std::fs::read_to_string("../input/day19.txt").unwrap();
    let (rules, values) = parse_rules(&input);
    println!(
        "Day 19.a: {}",
        values
            .iter()
            .filter(|x| matches(x, &rules, 0) == Some(""))
            .count()
    );

    println!(
        "Day 19.b: {}",
        values
            .iter()
            .filter(|x| loop_matches(x, &rules) == Some(""))
            .count()
    );
}
