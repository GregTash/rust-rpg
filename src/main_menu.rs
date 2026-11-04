use std::io;
use crate::player::Player;

pub fn main_menu(player: &mut Player) {
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

    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line!");

        match input.trim() {
            "1" => { new_game(player); break; },
            "2" => { load_game(); break; },
            "3" => { exit_game(); break; },
            _ => println!("Please choose a valid option!"),
        }
    }
}

pub fn new_game(player: &mut Player) {
    println!("What is your name?");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input)
    .expect("Could not read line!");

    player.name = input.trim().to_string();    
}

pub fn load_game() {
    println!("Load WIP!");
}

pub fn exit_game() {
    println!("Thanks for playing!");
    std::process::exit(0);
}