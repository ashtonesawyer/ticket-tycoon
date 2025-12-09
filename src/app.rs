use crate::currency::WalletError;
use crate::game::*;
use crate::ticket::*;
use crate::upgrade::*;

use dioxus::{logger::tracing::info, prelude::*};
use std::time::Duration;
use tokio::time::sleep;

pub fn app() -> Element {
    let mut state = use_signal(GameState::new);
    let mut error = use_signal(String::new);
    let mut show = use_signal(|| false);

    if state.read().working().is_empty() {
        state.write().init_queue();
    }

    use_future(move || async move {
        loop {
            state.write().autosolve();
            sleep(Duration::from_secs(1)).await;
        }
    });

    rsx! {
        Header { 
            cash: state.read().wallet().cash(), 
            xp: state.read().wallet().xp(),
            on_input:  move |_| show.set(!show()),
        }
        div {
            style: "display: flex; flex-direction: row; justify-content: space-around; padding: 15px 5px; min-height: 385px",

            Queue {
                queue: state.read().working().to_vec(),
                on_click: move |i| {
                    error.set(String::new());
                    state.write().click_ticket(i)
                }
            }
            Upgrades {
                upgrades: state.read().avail_upgrades(),
                on_click: move |id| {
                    error.set(String::new());
                    match state.write().buy_upgrade(&id) {
                        Ok(()) => {},
                        Err(BuyError::UpgradeUnavailable) => panic!("Should check for availability before showing to user"),
                        Err(BuyError::Wallet(WalletError::InsufficientCash)) => error.set("Not enough cash to buy this upgrade".to_string()),
                        Err(BuyError::Wallet(WalletError::InsufficientXP)) => error.set("Not enough XP to buy this upgrade".to_string()),
                    }
                }
            }
        }
        div {
            style: "border-bottom: 1px solid black; padding-top: 30px",
        }
        Error { err: error.read() }
        if show() {
            Stat { stats: state.read().stats() }
        }
    }
}

#[component]
fn Header(cash: u64, xp: u64, on_input: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "padding: 0px 30px 15px 15px; display: flex; flex-direction: row; justify-content: space-between; border-bottom: 1px solid black;",

            span {
                    label { "show stats" }
                    input {
                        r#type: "checkbox",
                        oninput: move |_| on_input.call(()),
                    }
                }
            span { "[ ${cash} ]  [ {xp} XP ]" }
        }
    }
}

#[component]
fn Error(err: String) -> Element {
    rsx! {
        div {
            style: "padding: 15px; color: red; text-align: center; font-weight: bold",

            "{err}"
        }
    }
}

#[component]
fn Queue(queue: Vec<Ticket>, on_click: EventHandler<usize>) -> Element {
    rsx! {
        div {
            style: "width: 375px; display: flex; flex-direction: column; gap: 10px;",

            h3 { "Ticket Queue" }

            for (i, ticket) in queue.iter().enumerate() {
                {
                    let pct = (ticket.clicked() as f32 / ticket.goal() as f32 * 100.0).min(100.0);

                    rsx! {
                        div {
                            style: "border: 1px solid black; padding: 5px; display: flex; flex-direction: column; gap: 20px;",
                            key: "{i}",

                            div {
                                style: "display: flex; justify-content: space-between;",

                                span { "{ticket.name()}" }
                                span { {format!("{:?} - {:?}", ticket.category(), ticket.difficulty()) }}
                            }
                            div {
                                style: "display: flex; justify-content: space-between;",

                                ProgBar { pct }
                                {
                                    let text = if pct == 100.0 { "Close" } else { "Work" };
                                    rsx! {
                                        button {
                                            onclick: move |_| on_click.call(i),
                                            "{text}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProgBar(pct: f32) -> Element {
    let width = 20;
    let filled = ((pct / 100.0) * width as f32).round() as usize;
    let empty = width - filled;

    let bar = format!(
        "[{}{}] {}%",
        "#".repeat(filled),
        ".".repeat(empty),
        pct as usize,
    );

    rsx! {
        div {
            style: "font-family: monospace;",
            "{bar}"
        }
    }
}

#[component]
fn Upgrades(upgrades: Vec<Upgrade>, on_click: EventHandler<String>) -> Element {
    rsx! {
        div {
            style: "width: 310px; display: flex; flex-direction: column; gap: 10px",

            h3 { "Upgrades" }
            for (i, upgrade) in upgrades.iter().enumerate() {
                {
                    let id = upgrade.id.clone();
                    rsx! {
                        div {
                            style: "border: 1px solid black; padding: 5px;",
                            key: "{i}",

                            h4 { "{upgrade.name}" }
                            p  { "{upgrade.desc}" }
                            div {
                                style: "display: flex; flex-direction: row; justify-content: space-between",

                                {
                                    let cost = match (upgrade.cost.cash(), upgrade.cost.xp()) {
                                        (0, 0) => "".to_string(),
                                        (x, 0) => format!("${}", x),
                                        (0, x) => format!("{} XP", x),
                                        (x, y) => format!("${} and {} XP", x, y),
                                    };
                                    rsx! {
                                        span { "{cost}" }
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
        }
    }
}

#[component]
fn Stat(stats: Stats) -> Element {
    info!("Stats: {:?}", stats);

    let autosolve = stats.autosolve
        .iter()
        .map(|(diff, cat)| format!("{:?}:{:?}", diff, cat))
        .collect::<Vec<_>>()
        .join(", ");
    let timestamp = chrono::Utc::now().format("%H:%M:%SZ");

    rsx! {
        pre {
            style:"font-family: monospace; font-size: 13px; ",

            "[{timestamp} DEBUG stats] click_mult={stats.multiplier:.2}\n",
            "[{timestamp} DEBUG stats] cash_mult={stats.cash_mult:.2}\n",
            "[{timestamp} DEBUG stats] xp_mult={stats.xp_mult:.2}\n",
            "[{timestamp} DEBUG stats] autosolve=[{autosolve}]"
        }
    }
}
