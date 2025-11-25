use crate::currency::WalletError;
use crate::game::*;
use crate::ticket::*;
use crate::upgrade::*;

use dioxus::prelude::*;

pub fn app() -> Element {
    let mut state = use_signal(|| GameState::new());
    let mut error = use_signal(|| String::new());

    if state.read().working().len() == 0 {
        state.write().init_queue();
    }

    rsx! {
        Header { cash: state.read().wallet().cash(), xp: state.read().wallet().xp() }
        Error { err: error.read() }
        Queue { queue: state.read().working().to_vec(), on_click: move |i| {
            error.set(String::new());
            state.write().click_ticket(i)
            }
        }
        Upgrades { upgrades: state.read().avail_upgrades(), on_click: move |id| {
            error.set(String::new());
            match state.write().buy_upgrade(&id) {
                Ok(()) => {return},
                Err(BuyError::UpgradeUnavailable) => panic!("Should check for availability before showing to user"),
                Err(BuyError::Wallet(WalletError::InsufficientCash)) => error.set("Not enough cash to buy this upgrade".to_string()),
                Err(BuyError::Wallet(WalletError::InsufficientXP)) => error.set("Not enough XP to buy this upgrade".to_string()),
            }
        }}
    }
}

#[component]
fn Header(cash: u64, xp: u64) -> Element {
    rsx! {
        span { "Cash: {cash} | XP: {xp}" }
    }
}

#[component]
fn Error(err: String) -> Element {
    rsx! {
        div {
            "{err}"
        }
    }
}

#[component]
fn Queue(queue: Vec<Ticket>, on_click: EventHandler<usize>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",

            for (i, ticket) in queue.iter().enumerate() {
                {
                    let pct = (ticket.clicked() as f32 / ticket.goal() as f32 * 100.0).min(100.0);

                    rsx! {
                        div {
                            key: "{i}",
                            p { "{ticket.name()}" }
                            p { {format!("{:?} | {:?} | {pct:.0}%", ticket.category(), ticket.difficulty()) }}
                            button {
                                onclick: move |_| on_click.call(i),
                                "Work"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Upgrades(upgrades: Vec<Upgrade>, on_click: EventHandler<String>) -> Element {
    rsx! {
        div {
            for (i, upgrade) in upgrades.iter().enumerate() {
                {
                    let id = upgrade.id.clone();
                    rsx! {
                        div {
                            key: "{i}",
                            h3 { "{upgrade.name}" }
                            p  { "{upgrade.desc}" }
                            button {
                                onclick: move |_| on_click.call(id.clone()),
                                "Buy"
                            }
                        }
                    }
                }
            }
        }
    }
}
