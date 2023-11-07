use shared::models;
use wasm_bindgen::JsValue;

use yew::prelude::*;

use crate::{components::*, invoke};

#[derive(Default)]
pub struct HistoryView {
    history: Vec<models::Entry>,
}

pub enum Message {
    Update(Vec<models::Entry>),
}

impl yew::Component for HistoryView {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        // get word history by IDs. We still will need to find these in the dictionary with another event
        ctx.link().send_future(async move {
            let results: Vec<models::Entry> =
                serde_wasm_bindgen::from_value(invoke("history_index", JsValue::null()).await)
                    .unwrap();

            Message::Update(results)
        });
        Self::default()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <Split<models::Entry>
                items={self.history.clone()}
                render_list={|entry: models::Entry|
                    html! {
                        <>
                            <Ruby entry={entry.clone()} />
                            <Definition definition={entry.definition} limit={3} />
                        </>
                    }
                }
                render_split={|entry: models::Entry|
                    html! {
                        <super::EntryView id={entry.id as u32}>
                        </super::EntryView>
                    }
                }
            />
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Update(history) => {
                self.history = history;
                true
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub hide_header: bool,
}

// For whatever reason in Tauri we need to wrap the command in another key?
// fn encapsulate_command(action: ) {}
