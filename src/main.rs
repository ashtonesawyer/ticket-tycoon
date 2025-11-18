mod app;
mod currency;
mod game;
mod ticket;
mod upgrade;

use dioxus;

fn main() {
    dioxus::launch(app::app);
}
