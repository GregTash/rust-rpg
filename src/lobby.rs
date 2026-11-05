use std::io;
use crate::global::Global;

pub fn lobby(global: &mut Global) {
    let mut input: String = String::new();

    loop {
        println!("Welcome to the lobby!");
        println!("1) Fight");
        println!("2) Show stats");
        println!("3) Exit");

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line!");

        match input.trim() {
            "1" => println!("Fight!"),
            "2" => global.player.show_stats(),
            "3" => { break; }
            _ => println!("Please enter valid input!"),
        }
    }
}