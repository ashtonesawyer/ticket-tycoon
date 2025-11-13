use crate::currency::*;
use crate::ticket::*;

use rand::Rng;

/// Data needed for the main game loop
pub struct GameState {
    /// Queue of unfinished tickets
    queue: Vec<Ticket>,
    /// How much of each currency the player has
    wallet: Currency,
    /// Tickets that are currently being worked on
    working: Vec<Ticket>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            wallet: Currency::new(),
            working: Vec::new(),
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
            ticket.click();
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
