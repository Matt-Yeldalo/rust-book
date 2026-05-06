use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Input your guess");

    let number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count: i8 = 0;

    loop {
        if guess(number, &mut guess_count) {
            break;
        }
    }
}

fn guess(answer: i32, guess_count: &mut i8) -> bool {
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // let guess: i32 = guess.trim().parse::<i32>().expect("Something went wrong");
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return false;
        }
    };

    if guess == answer {
        println!("You guessed it right in {} tries!", guess_count);
        return true;
    }

    *guess_count += 1;
    if guess < answer {
        println!("Too low!");
    } else {
        println!("Too high!");
    }

    return false;
}
