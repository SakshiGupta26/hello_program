use std::io;
use rand::prelude::*;

fn main() {
    let guess_list = [
        "apple", "banana", "cherry",
        "date", "fig", "grape", "kiwi", "lemon",
        "mango", "nectarine", "orange", "papaya",
        "quince", "raspberry", "strawberry",
        "tangerine", "ugli fruit", "voavanga", "watermelon",
        "xigua", "yellow passion fruit", "zucchini"
    ];

    let mut rng = thread_rng();
    let index = rng.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];

    println!("Random fruit: {}", random_fruit);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let fruit_selected = input.trim().to_lowercase();
            println!("You typed: {}", fruit_selected);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

//cd Guess_word