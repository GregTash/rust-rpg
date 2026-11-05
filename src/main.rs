mod global;
mod player;
mod main_menu;
mod statistics;
mod lobby;

use crate::global::Global;
use crate::player::Player;
use crate::main_menu::main_menu;
use crate::statistics::Statistics;
use crate::lobby::lobby;

fn main() {
    let mut global: Global = Global{
        player: Player{name: String::from(""), stats: Statistics {health: 20, damage: 5}},
    };
    main_menu(&mut global);

    lobby(&mut global);
}
