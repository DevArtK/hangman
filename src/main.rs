extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;


const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    is_revealed: bool
}


enum GameProgress {
    InProgress,
    Won,
    Lost
}


fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();

    let mut letters = breakup_word(&selected_word);

    loop {
        println!("You have [{}] turns left", turns_left);
        display_progress(&letters);

        println!("Enter a lettter to guess");
        let user_char = read_user_input_character();

        // Exit if user enters an asterisk

        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed = false; // if at least one letter is revealed won't lost a turn

        // Update the revealed state of each letter, if the user has guessed at least one letter correctly
        for letter in letters.iter_mut() { // mutable so we can change the value or revealed if correct char
            if letter.character == user_char {
                letter.is_revealed = true;
                at_least_one_revealed = true;
            }
        }

        // If user guessed incorrectly lose a turn
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,   // Restarts the loop
            GameProgress::Won => {
                println!("\nCongrats, you won! The word was : {}", selected_word);
                break;
            },
            GameProgress::Lost => {
                println!("\nSorry, you lost ! The word was : {}", selected_word);
                break;
            }
        }
    }


    display_progress(&letters);
    println!("{}", selected_word);

}


fn select_word() -> String {
    // Open file
    let mut file: File = File::open("./words.txt").expect("Could not open file");

    // Load contents of file into variable
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Error while reading contents of file to string");

    // Split the comma seperate String into a vector
        // Will store the words in Vecotr of string literals
    let words: Vec<&str> = contents
        .trim()
        .split(',')
        .collect();

    // Generate a random index
    let rand_i = rand::thread_rng().gen_range(0 .. words.len());

    // Return random workd from the contents of file
    return String::from(words[rand_i]);
}


fn breakup_word(word: &String) -> Vec<Letter> {
    // Create empty vector that will hold an list of Letter structs
    let mut letters: Vec<Letter> = Vec::new();

    // Wrap each character in a Letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            is_revealed: false
        })
    }

    return letters;
}


fn display_progress(letters: &Vec<Letter>) {
   // Loop through each letter and print out appropiate symbol
    let mut display_string = String::from("Progress: ");

    // Display appropriate character ( either a letter or _ ) for each letter in the letters vector
    for letter in letters {
        display_string.push(' '); // space the word out

        if letter.is_revealed {
            display_string.push(letter.character);

        } else {
            display_string.push('_');
        }
        display_string.push(' ');

    }

    println!("{}", display_string);
}



fn read_user_input_character() -> char {
    // Create variable to store user input
    let mut input = String::new();

    // Get user input
    match io::stdin().read_line(&mut input) {
        // If success, match the input for the characters input from user, and grab the first character
        Ok(_) => {
            // Take the first character if a string is passed in
            match input.chars().next() {
                Some(c) => { return c; } // IF success (there was a character) return that character
                None => { return '*'; } // IF failure return '*'
            }
        }
        Err(_) => { return '*'; }
        }
}



// Check if game is in progress, won, or lost
fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    // Determine if all letters have been revealed
    let mut all_revealed = true;
    for letter in letters {
        if !letter.is_revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won
    }

    // If turns left,and at least one is not revealed  game is still going
    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
