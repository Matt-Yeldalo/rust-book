fn main() {
    println!("Guess the number");
    println!("Input your guess");

    let number: i32 = 99;

    while !guess(number) {}
}

fn guess(answer: i32) -> bool {
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let final_guess: i32 = guess.trim().parse::<i32>().expect("Something went wrong");

    if final_guess == answer {
        println!("You got lucky");
        return true;
    } else {
        println!("Wrong, try again");
        return false;
    }
}
