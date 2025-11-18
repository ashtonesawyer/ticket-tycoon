use crate::currency::*;
use crate::ticket::*;

use rand::Rng;

#[test]
fn empty() {
    let game = GameState::new();
    assert_eq!(game.queue.len(), 0);
    assert_eq!(game.wallet.cash(), 0);
    assert_eq!(game.wallet.xp(), 0);
    assert_eq!(game.working.len(), 0);
}

#[test]
fn random_tickets() {
    let mut game = GameState::new();
    for _ in 0..5 {
        game.spawn_ticket();
    }
    for i in 0..5 {
        for j in (i + 1)..5 {
            assert!(game.queue[i] != game.queue[j]);
        }
    }
}

#[test]
fn click_easy_complete() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Easy, Category::Web, "name"));
    for _ in 0..6 {
        game.click_ticket(0);
    }
    assert_eq!(game.wallet.cash(), 10);
    assert_eq!(game.wallet.xp(), 5);
    assert_eq!(game.working.len(), 0);
}

#[test]
fn click_easy_incomplete() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Easy, Category::Web, "name"));
    for _ in 0..4 {
        game.click_ticket(0);
    }
    assert_eq!(game.wallet.cash(), 0);
    assert_eq!(game.wallet.xp(), 0);
    assert_eq!(game.working.len(), 1);
}

#[test]
fn click_multiplier_25() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Hard, Category::Web, "name"));
    game.multiplier += 0.25;
    for _ in 0..15 {
        game.click_ticket(0);
    }
    assert!(game.working[0].clicked() > 15);
    assert!(game.working[0].clicked() < 22);
}

#[test]
fn click_multiplier_50() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Hard, Category::Web, "name"));
    game.multiplier += 0.5;
    for _ in 0..15 {
        game.click_ticket(0);
    }
    assert!(game.working[0].clicked() > 20);
}

#[test]
fn click_multiplier_75() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Hard, Category::Web, "name"));
    game.multiplier += 0.75;
    for _ in 0..15 {
        game.click_ticket(0);
    }
    assert!(game.working[0].clicked() > 22);
}

/// Data needed for the main game loop
pub struct GameState {
    /// Queue of unfinished tickets
    queue: Vec<Ticket>,
    /// How much of each currency the player has
    wallet: Currency,
    /// Tickets that are currently being worked on
    working: Vec<Ticket>,
    /// How many "clicks" per user-click
    multiplier: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            wallet: Currency::new(),
            working: Vec::new(),
            multiplier: 1.0,
        }
    }

    pub fn wallet(&self) -> &Currency {
        &self.wallet
    }

    pub fn working(&self) -> &Vec<Ticket> {
        &self.working
    }

    pub fn multiplier_add(&mut self, amt: f32) {
        self.multiplier += amt;
    }

    pub fn init_queue(&mut self) {
        for _ in 0..4 {
            self.spawn_ticket();
            self.assign_next_ticket();
        }
    }

    /// Add a random new ticket to the queue
    pub fn spawn_ticket(&mut self) {
        let mut rng = rand::rng();
        let difficulty = match rng.random_range(0..3) {
            0 => Difficulty::Easy,
            1 => Difficulty::Med,
            2 => Difficulty::Hard,
            _ => panic!("Random number generated outside of range"),
        };
        let category = match rng.random_range(0..5) {
            0 => Category::Network,
            1 => Category::Windows,
            2 => Category::Linux,
            3 => Category::Web,
            4 => Category::Misc,
            _ => panic!("Random number generated outside of range"),
        };
        let name = format!("{:?} issue #{:04}", category, rng.random_range(1000..9999));
        self.queue.push(Ticket::new(difficulty, category, &name));
    }

    /// Move one from queue to working
    pub fn assign_next_ticket(&mut self) {
        if let Some(ticket) = self.queue.pop() {
            self.working.push(ticket);
        }
    }

    /// Process a click on a ticket
    pub fn click_ticket(&mut self, index: usize) {
        if let Some(ticket) = self.working.get_mut(index) {
            let mut rng = rand::rng();
            let clicks: u16 = if rng.random::<f32>() < (self.multiplier - 1.0) {
                (1.0 * self.multiplier).ceil() as u16
            } else {
                (1.0 * self.multiplier).floor() as u16
            };
            ticket.click(clicks);
            if ticket.is_complete() {
                let (cash, xp) = match ticket.difficulty() {
                    Difficulty::Easy => (10, 5),
                    Difficulty::Med => (25, 10),
                    Difficulty::Hard => (60, 20),
                };
                self.wallet.add_cash(cash);
                self.wallet.add_xp(xp);
            }
        }
        // Remove finished tickets
        self.working.retain(|t| !t.is_complete());
    }
}
