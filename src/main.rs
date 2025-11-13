mod currency;
mod game;
mod ticket;

use currency::*;
use game::*;
use ticket::*;

fn main() {
    println!("Hello, world!");
    let ticket = Ticket::new(Difficulty::Easy, Category::Network, "name");
}
