#[test]
fn easy_network() {
    let ticket = Ticket::new(Difficulty::Easy, Category::Network, "name1");
    assert_eq!(ticket.difficulty(), &Difficulty::Easy);
    assert_eq!(ticket.category(), &Category::Network);
    assert_eq!(ticket.clicked(), 0);
}

#[test]
fn hard_web() {
    let ticket = Ticket::new(Difficulty::Hard, Category::Web, "name2");
    assert_eq!(ticket.difficulty(), &Difficulty::Hard);
    assert_eq!(ticket.category(), &Category::Web);
    assert_eq!(ticket.clicked(), 0);
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

#[derive(Debug)]
pub struct Currency {
    cash: u64,
    xp: u64,
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
    pub fn new(difficulty: Difficulty, category: Category, name: &str) -> Self {
        Self {
            difficulty,
            category,
            clicked: 0,
            name: name.to_string(),
        }
    }

    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn clicked(self) -> u16 {
        self.clicked
    }
}