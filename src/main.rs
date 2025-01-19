use std::sync::Arc;
use tokio::sync::Mutex;
use crate::dungeon::{handle_player_input, run_dungeon, Dungeon};

mod entities;
mod dungeon;
mod event;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dungeon = Arc::new(Mutex::new(Dungeon::new(20, 20)));
    tokio::spawn(handle_player_input(Arc::clone(&dungeon)));
    run_dungeon(dungeon).await;

    Ok(())
}
