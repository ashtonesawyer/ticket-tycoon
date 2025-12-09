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
            effects: vec![Effects::IncMultiplier(1.12)]
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
#[ignore]
fn click_multiplier_25() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Hard, Category::Web, "name"));
    game.multiplier += 0.25;
    let clicks = Ticket::HARD_GOAL / 2; //needs to be small enough that it won't complete the ticket
    for _ in 0..clicks {
        game.click_ticket(0);
    }
    assert!(game.working[0].clicked() > clicks);
    assert!(game.working[0].clicked() < (clicks as f32 * 1.5).floor() as u16);
}

#[test]
#[ignore]
fn click_multiplier_50() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Hard, Category::Web, "name"));
    game.multiplier += 0.5;
    let clicks = Ticket::HARD_GOAL / 2;
    for _ in 0..clicks {
        game.click_ticket(0);
    }
    assert!(game.working[0].clicked() > (clicks as f32 * 1.25).ceil() as u16);
}

#[test]
#[ignore]
fn click_multiplier_75() {
    let mut game = GameState::new();
    game.working
        .push(Ticket::new(Difficulty::Hard, Category::Web, "name"));
    game.multiplier += 0.75;
    let clicks = Ticket::HARD_GOAL / 2;
    for _ in 0..clicks {
        game.click_ticket(0);
    }
    assert!(game.working[0].clicked() > (clicks as f32 * 1.5).ceil() as u16);
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

#[test]
fn autosolve_none() {
    let mut game = GameState::new();
    game.init_queue();
    for _ in 0..30 {
        game.autosolve();
    }
    for ticket in game.working {
        assert_eq!(ticket.clicked(), 0);
    }
}

#[test]
fn autosolve_one() {
    let mut game = GameState::new();
    game.init_queue();
    let diff = game.working[0].difficulty();
    let cat = game.working[0].category();
    let _ = game.autosolve.insert((*diff, *cat));
    for _ in 0..4 {
        game.autosolve();
    }
    assert_eq!(game.working[0].clicked(), 4);
}

#[test]
fn autosolve_dup() {
    let mut game = GameState::new();
    game.init_queue();
    game.working.push(game.working[0].clone());
    let diff = game.working[0].difficulty();
    let cat = game.working[0].category();
    let _ = game.autosolve.insert((*diff, *cat));
    for _ in 0..4 {
        game.autosolve();
    }
    assert_eq!(game.working[0].clicked(), 4);
    assert_eq!(game.working[4].clicked(), 4);
}

#[test]
fn autosolve_two() {
    let mut game = GameState::new();
    game.init_queue();
    let diff = game.working[0].difficulty();
    let cat = game.working[0].category();
    let diff1 = game.working[1].difficulty();
    let cat1 = game.working[1].category();
    let _ = game.autosolve.insert((*diff, *cat));
    _ = game.autosolve.insert((*diff1, *cat1));
    for _ in 0..4 {
        game.autosolve();
    }
    assert_eq!(game.working[0].clicked(), 4);
    assert_eq!(game.working[1].clicked(), 4);
}

#[test]
#[ignore]
fn cash_multiplier_25() {
    let mut game = GameState::new();
    game.cash_mult += 0.25;
    for _ in 0..8 {
        game.working
            .push(Ticket::new(Difficulty::Easy, Category::Web, "name"));
    }
    for _ in 0..8 {
        for _ in 0..=game.working[0].goal() {
            game.click_ticket(0);
        }
    }
    assert_eq!(game.working.len(), 0);
    assert_eq!(game.wallet.xp(), GameState::BASE_EASY_XP * 8);
    assert!(game.wallet.cash() > (GameState::BASE_EASY_CASH as f32 * 1.25).floor() as u64 * 8);
    assert!(game.wallet.cash() < (GameState::BASE_EASY_CASH as f32 * 1.25).ceil() as u64 * 8)
}

#[test]
#[ignore]
fn xp_multiplier_25() {
    let mut game = GameState::new();
    game.xp_mult += 0.25;
    for _ in 0..8 {
        game.working
            .push(Ticket::new(Difficulty::Easy, Category::Web, "name"));
    }
    for _ in 0..8 {
        for _ in 0..=game.working[0].goal() {
            game.click_ticket(0);
        }
    }
    assert_eq!(game.working.len(), 0);
    assert_eq!(game.wallet.cash(), GameState::BASE_EASY_CASH * 8);
    assert!(game.wallet.xp() > (GameState::BASE_EASY_XP as f32 * 1.25).floor() as u64 * 8);
    assert!(game.wallet.xp() < (GameState::BASE_EASY_XP as f32 * 1.25).ceil() as u64 * 8)
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

/// Multiply the base by the multiplier and round up/down
/// Should round up ((multiplier % 1) * 100)% of the time
fn rand_round(base: u64, multiplier: f32) -> u64 {
    let mut rng = rand::rng();
    if rng.random::<f32>() < (multiplier % 1.0) {
        (base as f32 * multiplier).ceil() as u64
    } else {
        (base as f32 * multiplier).floor() as u64
    }
}

/// Struct for returning game stats affected by upgrades
#[derive(Clone, PartialEq, Debug)]
pub struct Stats {
    pub multiplier: f32,
    pub cash_mult: f32,
    pub xp_mult: f32,
    pub autosolve: HashSet<(Difficulty, Category)>,
}

/// Data needed for the main game loop
#[derive(Clone, Debug)]
pub struct GameState {
    /// Queue of unfinished tickets
    queue: Vec<Ticket>,
    /// How much of each currency the player has
    wallet: Currency,
    /// Tickets that are currently being worked on
    working: Vec<Ticket>,
    /// How many "clicks" per user-click
    multiplier: f32,
    /// How much cash per completed ticket multiplier
    cash_mult: f32,
    /// How much XP per completed ticket multiplier
    xp_mult: f32,
    /// What difficulty + category combos have autosolve enabled
    autosolve: HashSet<(Difficulty, Category)>,
    /// All possible upgrades mapped by ID
    upgrades: HashMap<String, Upgrade>,
    /// ID of any purchased upgrades
    purchased: HashSet<String>,
}

impl GameState {
    const BASE_EASY_CASH: u64 = 10;
    const BASE_EASY_XP: u64 = 5;
    const BASE_MED_CASH: u64 = 25;
    const BASE_MED_XP: u64 = 10;
    const BASE_HARD_CASH: u64 = 60;
    const BASE_HARD_XP: u64 = 20;

    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            wallet: Currency::new(),
            working: Vec::new(),
            multiplier: 1.0,
            cash_mult: 1.0,
            xp_mult: 1.0,
            autosolve: HashSet::new(),
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

    pub fn stats(&self) -> Stats {
        Stats {
            multiplier: self.multiplier,
            cash_mult: self.cash_mult,
            xp_mult: self.xp_mult,
            autosolve: self.autosolve.clone(),
        }
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
        let difficulty = match rng.random_range(0..10) {
            0..=3 => Difficulty::Easy,
            4..=6 => Difficulty::Med,
            7..=10 => Difficulty::Hard,
            _ => panic!("Random number generated outside of range"),
        };
        let category = match rng.random_range(0..20) {
            0..=6 => Category::Misc,
            7..=10 => Category::Windows,
            11..=13 => Category::Linux,
            14..=16 => Category::Network,
            17..=20 => Category::Web,
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
            let clicks = rand_round(1, self.multiplier) as u16;
            ticket.click(clicks);
            if ticket.is_complete() {
                let (cash, xp) = match ticket.difficulty() {
                    Difficulty::Easy => (
                        rand_round(Self::BASE_EASY_CASH, self.cash_mult),
                        rand_round(Self::BASE_EASY_XP, self.xp_mult),
                    ),
                    Difficulty::Med => (
                        rand_round(Self::BASE_MED_CASH, self.cash_mult),
                        rand_round(Self::BASE_MED_XP, self.xp_mult),
                    ),
                    Difficulty::Hard => (
                        rand_round(Self::BASE_HARD_CASH, self.cash_mult),
                        rand_round(Self::BASE_HARD_XP, self.xp_mult),
                    ),
                };
                self.wallet.add_cash(cash);
                self.wallet.add_xp(xp);

                // Remove finished tickets
                let _ = self.working.remove(index);
            }
        }
    }

    /// Click once on any ticket that is available and matches the currently
    /// bought autosolve upgrades
    pub fn autosolve(&mut self) {
        for (diff, cat) in &self.autosolve {
            for ticket in &mut self.working {
                if ticket.difficulty() == diff && ticket.category() == cat {
                    ticket.click(1);
                }
            }
        }
    }

    /// Check if an upgrade is available to buy
    /// It's available to buy if:
    /// - It exists in the upgrade hashmap
    /// - It has not already been purchased
    /// - All of its prerequisite purchases have been made
    ///
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

        self.upgrades
            .get(id)
            .unwrap()
            .requires
            .iter()
            .all(|req| self.purchased.contains(req))
    }

    /// Get a list of currently available upgrades
    pub fn avail_upgrades(&self) -> Vec<Upgrade> {
        let mut avail = Vec::new();

        for (key, val) in self.upgrades.iter() {
            if self.upgrade_available(key) {
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
            self.wallet.spend(upgrade.cost.cash(), upgrade.cost.xp())?;
            self.purchased.insert(upgrade.id.clone());
            upgrade.effects.clone()
        };
        self.apply_upgrade(&effects);

        Ok(())
    }

    /// Update the GameStruct with the provided effects
    /// Should only be called from buy_upgrade()
    fn apply_upgrade(&mut self, effects: &Vec<Effects>) {
        for up in effects {
            match up {
                Effects::IncMultiplier(x) => self.multiplier *= x,
                Effects::AutoSolve(diff, cat) => {
                    let _ = self.autosolve.insert((*diff, *cat));
                }
                Effects::IncCashMultiplier(x) => self.cash_mult *= x,
                Effects::IncXPMultiplier(x) => self.xp_mult *= x,
            }
        }
    }
}
