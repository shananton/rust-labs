fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    use std::io::{stdin, stdout, Write};

    const MAX: u64 = 100;
    let number = 1 + random_int() % MAX;
    println!("Guess a number between 1 and {}", MAX);
    loop {
        print!("Your guess: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input: u64 = input.trim().parse().unwrap();
        if input < number {
            println!("Too low!")
        } else if input > number {
            println!("Too high!")
        } else {
            println!("Congratulations! You won!");
            return;
        }
    }
}
