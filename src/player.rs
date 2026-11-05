use crate::statistics::Statistics;

pub struct Player {
    pub name: String,
    pub stats: Statistics,
}

impl Player {
    pub fn show_stats(&self) {
        println!("{}", self.name);
        println!("{}", self.stats.health);
        println!("{}", self.stats.damage);
    }
}