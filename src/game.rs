use crate::currency::*;
use crate::ticket::*;
use crate::upgrade::*;

use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[test]
fn empty() {
    let game = GameState::new();
    assert_eq!(game.queue.len(), 0);
    assert_eq!(game.wallet.cash(), 0);
    assert_eq!(game.wallet.xp(), 0);
    assert_eq!(game.working.len(), 0);
}

#[test]
fn load_hashmap() {
    let game = GameState::new();
    assert!(game.upgrades.contains_key("ergonomic_mousepad"));
    let mut cost = Currency::new();
    cost.add_cash(90);
    assert_eq!(
        game.upgrades.get("ergonomic_mousepad"),
        Some(Upgrade {
            id: "ergonomic_mousepad".to_string(),
            name: "Ergonomic Mousepad".to_string(),
            desc: "Gel wrist support: for when you're 25 but feel 65.".to_string(),
            cost: cost,
            requires: vec!["slightly_less_terrible_mouse".to_string()],
            effects: Effects {
                inc_multiplier: 1.12
            }
        })
        .as_ref()
    );
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

#[test]
fn upgrade_happy() {
    let mut game = GameState::new();
    game.wallet.add_xp(30);
    assert!(
        game.buy_upgrade(&"wrist_stretch_reminder".to_string())
            .is_ok()
    );
    assert_eq!(game.wallet.xp(), 0);
}

#[test]
fn upgrade_unavail() {
    let mut game = GameState::new();
    assert!(
        game.buy_upgrade(&"fake_upgrade_id".to_string())
            .is_err_and(|x| match x {
                BuyError::UpgradeUnavailable => true,
                _ => false,
            })
    );
}

#[test]
fn upgrade_insuff_cash() {
    let mut game = GameState::new();
    assert!(
        game.buy_upgrade(&"slightly_less_terrible_mouse".to_string())
            .is_err_and(|x| match x {
                BuyError::Wallet(WalletError::InsufficientCash) => true,
                _ => false,
            })
    );
}

#[test]
fn upgrade_insuff_xp() {
    let mut game = GameState::new();
    assert!(
        game.buy_upgrade(&"wrist_stretch_reminder".to_string())
            .is_err_and(|x| match x {
                BuyError::Wallet(WalletError::InsufficientXP) => true,
                _ => false,
            })
    );
}

pub enum BuyError {
    Wallet(WalletError),
    UpgradeUnavailable,
}

impl From<WalletError> for BuyError {
    fn from(err: WalletError) -> Self {
        BuyError::Wallet(err)
    }
}

/// Read upgrades.json and return the contents
fn read_upgrades() -> Vec<Upgrade> {
    let file = File::open("src/upgrades.json").expect("Could not open upgrades.json");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Could not read upgrades.json");
    serde_json::from_str(&contents).unwrap()
}

/// Load upgrades from file and load into hashmap
fn load_upgrades() -> HashMap<String, Upgrade> {
    let upgrades = read_upgrades();
    let mut hash = HashMap::new();
    for upgrade in upgrades {
        hash.insert(upgrade.id.clone(), upgrade);
    }
    hash
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
    /// All possible upgrades mapped by ID
    upgrades: HashMap<String, Upgrade>,
    /// ID of any purchased upgrades
    purchased: HashSet<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            wallet: Currency::new(),
            working: Vec::new(),
            multiplier: 1.0,
            upgrades: load_upgrades(),
            purchased: HashSet::new(),
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

    /// Check if an upgrade is available to buy
    /// It's available to buy if:
    /// - It exists in the upgrade hashmap
    /// - It has not already been purchased
    /// - All of its prerequisite purchases have been made
    /// Note that it does **not** check if the upgrade can be afforded
    fn upgrade_available(&self, id: &String) -> bool {
        // already bought it, can't buy it again
        if self.purchased.contains(id) {
            return false;
        };
        // not there, can't buy it
        if !self.upgrades.contains_key(id) {
            return false;
        };

        let met_prereqs = self
            .upgrades
            .get(id)
            .unwrap()
            .requires
            .iter()
            .all(|req| self.purchased.contains(req));
        met_prereqs
    }

    /// Get a list of currently available upgrades
    pub fn avail_upgrades(&self) -> Vec<Upgrade> {
        let mut avail = Vec::new();

        for (key, val) in self.upgrades.iter() {
            if self.upgrade_available(&key) {
                avail.push(val.clone());
            }
        }
        avail
    }

    /// Buy an upgrade and apply its effects
    /// Will return an error if:
    /// - Upgrade isn't available
    /// - Upgrade is too expensive
    pub fn buy_upgrade(&mut self, id: &String) -> Result<(), BuyError> {
        // this checks if the id exists as a key, so unwrap() can be used safely later
        if !self.upgrade_available(id) {
            return Err(BuyError::UpgradeUnavailable);
        }

        let effects = {
            let upgrade = self.upgrades.get(id).unwrap();
            self.wallet.rm_cash(upgrade.cost.cash())?;
            self.wallet.rm_xp(upgrade.cost.xp())?;
            self.purchased.insert(upgrade.id.clone());
            upgrade.effects.clone()
        };
        self.apply_upgrade(&effects);

        Ok(())
    }

    /// Update the GameStruct with the provided effects
    /// Should only be called from buy_upgrade()
    fn apply_upgrade(&mut self, effects: &Effects) {
        self.multiplier *= effects.inc_multiplier;
    }
}
