#[test]
fn easy_network() {
    let ticket = Ticket::new(Difficulty::Easy, Category::Network, "name");
    assert_eq!(ticket.difficulty(), &Difficulty::Easy);
    assert_eq!(ticket.category(), &Category::Network);
    assert_eq!(ticket.clicked(), 0);
}

#[test]
fn hard_web() {
    let ticket = Ticket::new(Difficulty::Hard, Category::Web, "name");
    assert_eq!(ticket.difficulty(), &Difficulty::Hard);
    assert_eq!(ticket.category(), &Category::Web);
    assert_eq!(ticket.clicked(), 0);
}

#[test]
fn click_it() {
    let mut ticket = Ticket::new(Difficulty::Hard, Category::Web, "name");
    ticket.click();
    assert_eq!(ticket.clicked(), 1);
}

#[test]
fn click_it_many() {
    let mut ticket = Ticket::new(Difficulty::Easy, Category::Web, "name");
    for _ in 0..5 {
        ticket.click();
    }
    assert_eq!(ticket.clicked(), 5);
}

#[test]
fn not_complete() {
    let mut ticket = Ticket::new(Difficulty::Easy, Category::Web, "name");
    assert_eq!(ticket.is_complete(), false);
}

#[test]
fn easy_complete() {
    let mut ticket = Ticket::new(Difficulty::Easy, Category::Web, "name");
    for _ in 0..6 {
        ticket.click();
    }
    assert_eq!(ticket.is_complete(), true);
}

#[test]
fn med_complete() {
    let mut ticket = Ticket::new(Difficulty::Med, Category::Web, "name");
    assert_eq!(ticket.is_complete(), false);
    for _ in 0..16 {
        ticket.click();
    }
    assert_eq!(ticket.is_complete(), true);
}

#[test]
fn hard_complete() {
    let mut ticket = Ticket::new(Difficulty::Hard, Category::Web, "name");
    assert_eq!(ticket.is_complete(), false);
    for _ in 0..31 {
        ticket.click();
    }
    assert_eq!(ticket.is_complete(), true);
}

/// How difficult a ticket is to complete
#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Med,
    Hard,
}

/// Categories that tickets can full under
#[derive(Debug, PartialEq)]
pub enum Category {
    Network,
    Windows,
    Linux,
    Web,
    Misc,
}

/// Ticket object 
#[derive(Debug)]
pub struct Ticket {
    /// How hard the ticket is to complete
    difficulty: Difficulty,
    /// What kind of ticket it is
    category: Category,
    /// How many times the ticket has already been clicked
    clicked: u16,
    /// Name of the ticket
    name: String,
}

impl Ticket {
    /// Create a new ticket 
    pub fn new(difficulty: Difficulty, category: Category, name: &str) -> Self {
        Self {
            difficulty,
            category,
            clicked: 0,
            name: name.to_string(),
        }
    }

    /// Returns a reference to the difficulty
    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    /// Returns a reference to the category
    pub fn category(&self) -> &Category {
        &self.category
    }

    /// Returns how many time the ticket was clicked
    pub fn clicked(self) -> u16 {
        self.clicked
    }

    /// Click the ticket one time
    pub fn click(&mut self) {
        self.clicked += 1;
    }

    /// Check if a ticket has been clicked enough to be completed
    /// The higher the difficulty, the more the ticket needs to be clicked
    pub fn is_complete(&self) -> bool {
        match self.difficulty {
            Difficulty::Easy => self.clicked > 5,
            Difficulty::Med => self.clicked > 15,
            Difficulty::Hard => self.clicked > 30, 
        }
    }
}