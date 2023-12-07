// use std::io;
use std::io::{ Read, stdin };

fn main() {
    println!("Welcome to Hangman");

    let correct_word = fetch_word();

    let mut hint_vec: Vec<char> = Vec::new();
    for _ in 0..correct_word.len() {
        hint_vec.push('_');
    }

    let mut lives = 10;
    let mut guessed_correctly: bool;
    let mut wrong_letters: Vec<char> = Vec::new();
    while hint_vec.contains(&'_') && lives > 0 {
        guessed_correctly = false;
        println!("{:?}", hint_vec);
        println!("Guess a letter: ");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read guess");
        let mut guess = input.chars().nth(0).unwrap();
        guess.make_ascii_lowercase();

        for i in 0..correct_word.len() {
            if guess == correct_word.chars().nth(i).unwrap() {
                hint_vec[i] = guess;
                guessed_correctly = true;
            }
        }

        println!("--------------------------------------------------");
        if !guessed_correctly && !wrong_letters.contains(&guess) {
            lives -= 1;
            wrong_letters.push(guess);
            println!("Lives: {}", lives);
        }

        if wrong_letters.len() > 0 {
            println!("Wrong guesses: {:?}", wrong_letters)
        }
        
    }
    println!("{:?}", hint_vec);

    if lives > 0 {
        println!("You win! The word was: {}", correct_word);
    } else {
        println!("You lose. The word was {}", correct_word);
    }
}

fn fetch_word() -> String {
    let response = reqwest::blocking::get("https://random-word-api.vercel.app/api?words=1");
    let mut body = String::new();
    let _ = response.unwrap().read_to_string(&mut body);
    
    // Getting around using json deserialization by removing [""] from response string (im a dependency hater)
    body[2..body.len()-2].to_string()
}