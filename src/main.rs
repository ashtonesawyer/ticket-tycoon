mod currency;
mod game;
mod ticket;
mod app;

use dioxus;

fn main() {
    dioxus::launch(app::app);
}
