
fn parse() -> Vec<(String, Vec<(usize, String)>)> {
    std::fs::read_to_string("../input/day7.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (container, inventory) = line.split_at(line.find(" bags contain ").unwrap());
            let inventory = &inventory[" bags contain ".len()..];

            let inventory = if inventory == "no other bags." {
                Vec::new()
            } else {
                inventory.split(", ").map(|x| {
                    let (num, name) = x.split_at(x.find(' ').unwrap());
                    let name = &name[1..x.rfind(' ').unwrap()-1];
                    (num.parse::<usize>().unwrap(), name.to_string())
                }).collect::<Vec<(usize, String)>>()
            };
            (container.to_string(), inventory)
        }
        )
        .collect()
}


fn is_connected(adj: &Vec<&mut [bool]>, i: usize, j: usize, visited: &mut Vec<bool>) -> bool {
    if adj[i][j] {
        return true;
    }

    if visited[i] {
        return false;
    }

    visited[i] = true;

    for (vertex,value) in adj[i].iter().enumerate() {
        if *value && is_connected(adj, vertex, j, visited) {
            return true;
        }
    }

    false
}


fn part1() {
    
    let mut bags = parse();

    let mut matrix = vec![false; bags.len() * bags.len()];
    let mut adj : Vec<_> = matrix.as_mut_slice().chunks_mut(bags.len()).collect();


    bags.sort_by(|x, y| x.0.cmp(&y.0));

    for (index, (_bag, inventory)) in (&bags).iter().enumerate() {

        for (_num,name) in inventory {
            if let Ok(i) = bags.binary_search_by_key(&name.as_str(), |a| a.0.as_str()) {
                adj[index][i] = true;
            } else {
                println!("couldnt find {}", name);
            }
        }
    }

    let mut connected = 0;
    let mut visited = vec![false; bags.len()];
    if let Ok(shiny_gold) = bags.binary_search_by_key(&"shiny gold", |x| x.0.as_str()) {
        for (index, (_, inventory)) in (&bags).iter().enumerate() {
            if index == shiny_gold { continue; }
            for b in &mut visited {
                *b = false;
            }

            if inventory.len() > 0 && is_connected(&adj, index, shiny_gold, &mut visited) {
                connected += 1;
            }
        }
    } else {
        println!("couldnt find shiny gold bag.");
    }

    
    println!("Day 7.a: {}", connected);
}


fn count_bags(bag: &str, bags: &Vec<(String, Vec<(usize, String)>)>) -> usize {
    let mut num_bags = 1;

    if let Ok(index) = bags.binary_search_by_key(&bag, |x| x.0.as_str()) {        
        for (num, item) in &bags[index].1 {
            num_bags += count_bags(item.as_str(), bags) * num;
        }
    } else {
        panic!("Couldnt find bag.");
    }

    num_bags
}

fn part2() {
    
    let mut bags = parse();
    bags.sort_by(|x, y| x.0.cmp(&y.0));

    println!("Day 7.b: {}", count_bags("shiny gold", &bags)-1);
}

pub fn run() {
    part1();
    part2();
}