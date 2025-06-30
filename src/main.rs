use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::io; //to take input from the user

fn main() {
    let guess_list = ["grapes", "banana", "orange", "rasmalai"];
    let mut rng = thread_rng();

    let index = rng.gen_range(0..guess_list.len());

    let random_fruit = guess_list[index];
    println!("Random fruit:{}", random_fruit);

    let mut input = String::new();
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit Selected:{}", fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("fruit entered does not found ");
                    continue;
                }

                if guess_checker(&fruit_selected, random_fruit) {
                    println!("You are winner");
                    break;
                } else {
                    println!("Retry")
                }
            }
            Err(error) => {
                println!("Error:{}", error)
            }
        }
    }
}

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    return fruit_selected == random_fruit;
}
