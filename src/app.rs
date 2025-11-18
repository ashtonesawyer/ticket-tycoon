use crate::game::*;
use crate::ticket::*;

use dioxus::prelude::*;

pub fn app() -> Element {
    let mut state = use_signal(|| GameState::new());
    if state.read().working().len() == 0  {state.write().init_queue();}
    

    rsx! {
        Header { cash: state.read().wallet().cash(), xp: state.read().wallet().xp() }
        Queue { queue: state.read().working().to_vec(), on_click: move |i| {
            state.write().click_ticket(i)
            }
        } 
    }
}

#[component]
fn Header(cash: u64, xp: u64) -> Element {
    rsx! {
        span { "Cash: {cash} | XP: {xp}" }
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
