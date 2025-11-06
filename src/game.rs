use crate::ticket::*;
use crate::currency::*;

/// Data needed for the main game loop
pub struct GameState {
    /// Queue of unfinished tickets
    queue: Vec<Ticket>,
    /// How much of each currency the player has
    currency: Currency,
    /// Tickets that are currently being worked on
    working: Vec<Ticket>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            currency: Currency::new(),
            working: Vec::new()
        }
    }
}