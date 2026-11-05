use std::num::Saturating;

fn charge(mut level: Saturating<u8>, amount: u8) -> Saturating<u8> {
    level += amount;
    level
}

fn drain(mut level: Saturating<u8>, amount: u8) -> Saturating<u8> {
    level -= amount;
    level
}

fn factorial(n: u64) -> Option<u64> {
    let mut result = 1u64;
    for i in 1..=n {
        result = result.checked_mul(i)?;
    }
    Some(result)
}

fn main() {
    let mut level = Saturating(80u8);

    level = charge(level, 30).min(Saturating(100));
    println!("after charging: {level}%");
    level = drain(level, 130);
    println!("after a long day: {level}%");
    
    for n in [5, 20, 21] {
        match factorial(n) {
            Some(small) => println!("{n}! = {small}"),
            None => println!("{n} is too big for a u64"),
        }
    }
}
