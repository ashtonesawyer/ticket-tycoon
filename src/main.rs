/// How difficult a ticket is to complete
enum Difficulty {
    Easy,
    Med,
    Hard,
}

/// Categories that tickets can full under
enum Category {
    Network,
    Windows,
    Linux,
    Web,
    Misc,
}

struct Currency {
    cash: u64,
    xp: u64,
}

/// Ticket object 
struct Ticket {
    /// How hard the ticket is to complete
    difficulty: Difficulty,
    /// What kind of ticket it is
    category: Category,
    /// How many times the ticket has already been clicked
    clicked: u16,
    /// Name of the ticket
    name: String,
}

fn main() {
    println!("Hello, world!");
}
