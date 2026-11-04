pub struct Player {
    pub name: String,
    pub health: i32,
    pub damage: i32,
}

impl Player {
    pub fn show_stats(&self) {
        println!("{}", self.name);
        println!("{}", self.health);
        println!("{}", self.damage);
    }
}