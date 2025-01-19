use rand::Rng;

#[derive(Debug)]
pub enum Event {
    Trap(String),
    Treasure(String, u32), // Player name, amount
}

pub fn random_event(player_name: &str) -> Option<Event> {
    let mut rng = rand::rng();
    let roll = rng.random_range(0..100);
    match roll {
        0..=10 => Some(Event::Trap(format!("{} fell into a trap!", player_name))),
        11..=20 => Some(Event::Treasure(
            format!("{} found treasure!", player_name),
            rng.random_range(50..101),
        )),
        _ => None,
    }
}
