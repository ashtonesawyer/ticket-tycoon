mod app;
mod currency;
mod game;
mod ticket;

use dioxus;

fn main() {
    dioxus::launch(app::app);
}
