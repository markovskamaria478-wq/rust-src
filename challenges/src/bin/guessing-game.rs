use rand::random_range;
use std::io::stdin;

fn main() {
    let num = random_range(0..=10);

    println!("Take your first guess!");

    let mut guess = String::new();
    loop {
        guess.clear(); // cleans the buffer
        stdin().read_line(&mut guess).expect("failed to read line");
        let guess = guess.trim().parse::<u32>();

        match guess {
            Ok(some) => {
                if some == num {
                    println!("you win!");
                    break;
                } else {
                    println!("you suck. try again")
                }
            }
            Err(e) => {
                println!("not a valid number: {e}");
                break;
            }
        }
    }
}
