use std::io;
use crate::{global::Global, lobby::lobby};

pub fn main_menu(global: &mut Global) {
    loop {
    println!("
░▒▓██████▓▒░░▒▓███████▓▒░░▒▓████████▓▒░▒▓██████▓▒░ ░▒▓███████▓▒░      ░▒▓███████▓▒░░▒▓███████▓▒░ ░▒▓██████▓▒░  
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░     ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░             ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░     ░▒▓█▓▒░      ░▒▓█▓▒░             ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░        
░▒▓█▓▒▒▓███▓▒░▒▓███████▓▒░░▒▓██████▓▒░░▒▓█▓▒▒▓███▓▒░░▒▓██████▓▒░       ░▒▓███████▓▒░░▒▓███████▓▒░░▒▓█▓▒▒▓███▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░     ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░     ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ 
 ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓██████▓▒░░▒▓███████▓▒░       ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░       ░▒▓██████▓▒░  
    ");

    println!("1) New Game");
    println!("2) Load Game");
    println!("3) Exit");

    let mut input: String = String::new();

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line!");

        match input.trim() {
            "1" => { new_game(global); lobby(global); },
            "2" => { load_game(); lobby(global); },
            "3" => { exit_game(); lobby(global); },
            _ => println!("Please choose a valid option!"),
        }
    }
}

pub fn new_game(global: &mut Global) {
    println!("What is your name?");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input)
    .expect("Could not read line!");

    global.player.name = input.trim().to_string();    
}

pub fn load_game() {
    println!("Load WIP!");
}

pub fn exit_game() {
    println!("Thanks for playing!");
    std::process::exit(0);
}