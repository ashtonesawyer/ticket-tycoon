mod currency;
mod game;
mod ticket;
mod app;

use currency::*;
use game::*;
use ticket::*;
use app::*;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app::app);
}
