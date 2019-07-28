use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut max_try = 6;

    loop {
        if max_try == 0 {
            println!("It's over you dumb ass");
            break;
        }

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                // 그냥 프린트가 되넹?
                println!("Fuck you idiot: {}", e);
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                max_try -= 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                max_try -= 1;
            },
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }
}
