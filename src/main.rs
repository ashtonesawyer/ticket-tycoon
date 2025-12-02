mod app;
mod currency;
mod game;
mod ticket;
mod upgrade;

fn main() {
    dioxus::launch(app::app);
}
