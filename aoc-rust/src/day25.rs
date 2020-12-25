fn encryption_key(subject_key: i64, loops: usize) -> i64 {
    let mut value = 1i64;
    for _ in 0..loops {
        value = value.wrapping_mul(subject_key) % 20201227;
    }
    value
}

pub fn run() {
    let card = 9033205i64;
    let door = 9281649i64;
    let mut key = 1i64;

    for loops in 1.. {
        key = key.wrapping_mul(7) % 20201227;
        
        if key == card {
            key = encryption_key(door, loops);
            break;
        }
        else if key == door  {
            key = encryption_key(card, loops);
            break;
        }        
    }

    println!("Day 25: {}", key);

}