use crate::entities::{spawn_monster, spawn_player, SharedEntity};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct Dungeon {
    grid: Vec<Vec<Option<char>>>, // 2D grid of entities
    players: Vec<SharedEntity>,
    monsters: Vec<SharedEntity>,
    pub log: Arc<Mutex<VecDeque<String>>>, // Shared log for events
}

impl Dungeon {
    pub fn new(rows: usize, cols: usize) -> Self {
        // Spawn monsters randomly
        let mut rng = rand::rng();
        let monsters = (0..5)
            .map(|_| {
                let x = rng.random_range(1..rows);
                let y = rng.random_range(1..cols);
                spawn_monster(x, y)
            })
            .collect();

        Self {
            grid: vec![vec![None; cols]; rows],
            players: vec![spawn_player("Player1", 0, 0)],
            monsters,
            log: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
        }
    }

    pub async fn render_map(&self) {
        let mut stdout = stdout();

        // Clear the terminal
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        // Draw the grid
        for row in &self.grid {
            for cell in row {
                let char_to_display = match cell {
                    Some(c) => *c,
                    None => '.',
                };
                execute!(stdout, Print(char_to_display)).unwrap();
            }
            execute!(stdout, Print("\n")).unwrap();
        }

        // Draw the event log
        execute!(stdout, Print("\nEvents:\n")).unwrap();
        let log = self.log.lock().await;
        for event in log.iter() {
            execute!(stdout, Print(format!("{}\n", event))).unwrap();
        }

        stdout.flush().unwrap();
    }

    pub async fn update_grid(&mut self) {
        // Clear the grid
        for row in &mut self.grid {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }

        // Place players and monsters
        for player in &self.players {
            let player = player.lock().await;
            self.grid[player.y][player.x] = Some('P');
        }

        for monster in &self.monsters {
            let monster = monster.lock().await;
            self.grid[monster.y][monster.x] = Some('M');
        }
    }

    async fn move_player(&mut self, dx: isize, dy: isize) {
        let player = &self.players[0]; // Assume single-player for now
        let mut player = player.lock().await;

        let new_x = ((player.x as isize + dx).max(0) as usize).min(self.grid[0].len() - 1);
        let new_y = ((player.y as isize + dy).max(0) as usize).min(self.grid.len() - 1);

        player.x = new_x;
        player.y = new_y;
    }
}

pub async fn run_dungeon(dungeon: Arc<Mutex<Dungeon>>) {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // Spawn a task for rendering and logging
    let render_dungeon = Arc::clone(&dungeon);
    tokio::spawn(async move {
        loop {
            {
                let mut dungeon = render_dungeon.lock().await;
                dungeon.update_grid().await; // Update grid based on entity positions
                dungeon.render_map().await; // Render the map and event log
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
        }
    });

    // Spawn player and monster tasks (similar to before)

    // Process incoming events
    while let Some(event) = rx.recv().await {
        let mut dungeon = dungeon.lock().await;
        let mut log = dungeon.log.lock().await;

        // Add the event to the log
        if log.len() == 10 {
            log.pop_front(); // Remove the oldest entry if full
        }
        log.push_back(event);
    }
}

pub async fn handle_player_input(dungeon: Arc<Mutex<Dungeon>>) {
    // enable_raw_mode().unwrap(); // Enable raw mode

    loop {
        // Poll for key events
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent { code, kind, .. }) = read().unwrap() {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Up => {
                            let mut dungeon = dungeon.lock().await;
                            dungeon.move_player(0, -1).await;
                        }
                        KeyCode::Down => {
                            let mut dungeon = dungeon.lock().await;
                            dungeon.move_player(0, 1).await;
                        }
                        KeyCode::Left => {
                            let mut dungeon = dungeon.lock().await;
                            dungeon.move_player(-1, 0).await;
                        }
                        KeyCode::Right => {
                            let mut dungeon = dungeon.lock().await;
                            dungeon.move_player(1, 0).await;
                        }
                        KeyCode::Char('q') => {
                            // Quit the game
                            // disable_raw_mode().unwrap();
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}