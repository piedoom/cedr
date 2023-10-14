use crate::commands::{self};
use crate::views::{self, entry};
use crate::{components::*, Route};
use shared::models;
use shared::InputMethod;
use web_sys::{HtmlInputElement, HtmlOptionElement};
use yew::prelude::*;

use crate::{components, invoke, Settings};

pub struct SearchView {
    search: String,
    method: InputMethod,
    results: Option<Vec<models::Entry>>,
    settings: Settings,
}

// Define the possible messages which can be sent to the component
pub enum Message {
    SetSearch(String),
    RequestResults,
    UpdateResults(Vec<models::Entry>),
    ChangeInputMethod(InputMethod),
}

impl Component for SearchView {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            search: Default::default(),
            method: InputMethod::Auto,
            results: Default::default(),
            settings: Settings::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <topbar>
                    <search>
                        <input type="text" placeholder="Search" onkeyup={
                            ctx.link().callback(|e: KeyboardEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Message::SetSearch(input.value())
                            })} />
                        <select onchange={
                            ctx.link().callback(|e: Event| {
                                let option: HtmlOptionElement = e.target_unchecked_into();
                                Message::ChangeInputMethod(
                                match option.value().as_str() {
                                "automatic" => {InputMethod::Auto},
                                "english" => {InputMethod::English},
                                "pinyin" => {InputMethod::Pinyin},
                                "chinese" => {InputMethod::Chinese},
                                _ => unreachable!()
                            })})
                        }>
                            <option value="automatic" selected=true>{ "Auto" }</option>
                            <option value="english">{ "English" }</option>
                            <option value="pinyin">{ "Pinyin" }</option>
                            <option value="chinese">{ "中文" }</option>
                        </select>
                    </search>
                </topbar>
                {
                    match &self.results {
                        Some(results) => {
                            html! {
                                <List::<models::Entry> items={results.clone()} render={
                                    |entry: models::Entry| {
                                        html! {
                                            <Link to={Route::Entry{id: entry.term.id as u32}}>
                                                <Ruby term={entry.term}/>
                                                <Definitions definitions={entry.definitions}/>
                                            </Link>
                                        }
                                    }
                                }>
                                </List::<models::Entry>>
                            }
                            // results.iter().map(|term| {
                            //     html! {
                            //         <a href={format!("/term/{}", term.0.id)}>
                            //             <components::Term definition={term.1.clone()} term={term.0.clone()}></components::Term>
                            //         </a>
                            //     }
                            // }).collect::<Html>()
                        },
                        // Show the history view as a default
                        None => {
                            html! {
                                <views::HistoryView hide_header={true}></views::HistoryView>
                            }
                        },
                    }
                }
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetSearch(value) => {
                self.search = value.clone();
                ctx.link().send_message(Message::RequestResults);
                true
            }
            Message::RequestResults => {
                let search = self.search.clone();
                let method = self.method.clone();
                if !search.is_empty() {
                    ctx.link()
                        //command query(input_method, search).await
                        .send_future(async move {
                            let results: Vec<models::Entry> = serde_wasm_bindgen::from_value({
                                invoke(
                                    "query",
                                    serde_wasm_bindgen::to_value(&commands::Query {
                                        method,
                                        query: search,
                                        include_sentences: false,
                                    })
                                    .unwrap(),
                                )
                                .await
                            })
                            .unwrap();
                            Message::UpdateResults(results)
                        })
                }

                true
            }
            Message::UpdateResults(results) => {
                self.results = Some(results);
                true
            }
            Message::ChangeInputMethod(input_method) => {
                self.method = input_method;
                ctx.link().send_message(Message::RequestResults);
                true
            }
        }
    }
}

impl crate::views::View for SearchView {
    fn title(&self) -> Option<String> {
        Some("Search".into())
    }
}
