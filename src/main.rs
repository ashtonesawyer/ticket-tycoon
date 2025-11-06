mod game;
mod ticket;
mod currency;

use game::*;
use ticket::*;
use currency::*;

fn main() {
    println!("Hello, world!");
    let ticket = Ticket::new(Difficulty::Easy, Category::Network, "name");
}
