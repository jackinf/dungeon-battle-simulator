use std::sync::Arc;
use tokio::sync::{Mutex};
use rand::Rng;

pub struct Entity {
    pub name: String,
    pub health: u32,
    pub attack: u32,
    pub x: usize,
    pub y: usize,
}

impl Entity {
    pub fn new(name: &str, health: u32, attack: u32, x: usize, y: usize) -> Self {
        Entity {
            name: name.to_string(), health, attack, x, y
        }
    }

    pub fn move_randomly(&mut self, max_x: usize, max_y: usize) {
        use rand::Rng;
        let mut rng = rand::rng();
        let dx: i32 = rng.random_range(-1..=1);
        let dy: i32 = rng.random_range(-1..=1);

        self.x = ((self.x as i32 + dx).max(0) as usize).min(max_x - 1);
        self.y = ((self.y as i32 + dy).max(0) as usize).min(max_y - 1);
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

pub type SharedEntity = Arc<Mutex<Entity>>;

pub fn spawn_player(name: &str, x: usize, y: usize) -> SharedEntity {
    Arc::new(Mutex::new(Entity::new(name, 100, 15, x, y)))
}

pub fn spawn_monster(x: usize, y: usize) -> SharedEntity {
    let mut rng = rand::rng();

    let health = rng.random_range(50..=100);
    let attack = rng.random_range(10..=20);

    Arc::new(Mutex::new(Entity::new("monster", health, attack, x, y)))
}