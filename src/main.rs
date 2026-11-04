mod player;
mod main_menu;

use crate::player::Player;
use crate::main_menu::main_menu;

fn main() {
    let mut player = Player{name: String::from(""), health: 20, damage: 5};
    main_menu(&mut player);
    Player::show_stats(&player);
}
