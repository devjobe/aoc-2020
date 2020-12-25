use itertools::Itertools;
use std::collections::HashMap;

fn split_once<'a>(input: &'a str, pat: &str) -> (&'a str, &'a str) {
    match input.find(pat) {
        Some(index) => (&input[..index], &input[index + pat.len()..]),
        None => (input, ""),
    }
}

fn find_allergen<'a>(sus: &HashMap<&'a str, HashMap<&'a str, i32>>) -> Option<(&'a str, &'a str)> {
    for (allergen, list) in sus {
        let (&suspect, &max) = list.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

        if list.iter().filter(|(_, y)| **y == max).count() == 1 {
            return Some((allergen, suspect));
        }
    }
    None
}

pub fn run() {
    let input = std::fs::read_to_string("../input/day21.txt").unwrap();
    let mut sus = HashMap::new();
    let mut all = HashMap::new();
    for line in input.lines() {
        let (ingredients, allergens) = split_once(&line, " (contains ");

        let allergens = allergens.trim_end_matches(')');

        let allergens: Vec<&str> = allergens.split(", ").collect();

        for ingredient in ingredients.split_whitespace() {
            all.entry(ingredient).and_modify(|x| *x += 1).or_insert(1);
            for &allergen in &allergens {
                let list = sus.entry(allergen).or_insert(HashMap::new());
                list.entry(ingredient).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }

    let mut confirmed = Vec::new();
    while sus.len() > 0 {
        if let Some((allergen, suspect)) = find_allergen(&sus) {
            confirmed.push((allergen, suspect));
            sus.remove(allergen);
            for (_, list) in sus.iter_mut() {
                list.remove(suspect);
            }
            all.remove(suspect);
        }
    }

    confirmed.sort_by(|(x, _), (y, _)| x.cmp(y));

    println!("Day 21.a: {}", all.values().sum::<i64>());
    println!("Day 21.b: {}", confirmed.iter().map(|(_, y)| y).join(","));
}
