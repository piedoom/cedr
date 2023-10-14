use shared::models;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

use crate::{components::*, invoke, views::View, Route};

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

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                if !ctx.props().hide_header {
                    <Bar title={self.title()} ></Bar>
                }
                <List<models::Entry>
                    items={self.history.clone()}
                    render={|entry: models::Entry| {
                        html! {
                            <Link to={Route::Entry { id: entry.term.id as u32 }}>
                                <Ruby term={entry.term} />
                                <Definitions definitions={entry.definitions} />
                            </Link>
                        }
                    }}>
                </List<models::Entry>>
            </>
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

impl crate::views::View for HistoryView {
    fn title(&self) -> Option<String> {
        Some("History".into())
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub hide_header: bool,
}

// For whatever reason in Tauri we need to wrap the command in another key?
// fn encapsulate_command(action: ) {}
