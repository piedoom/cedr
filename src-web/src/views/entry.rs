use crate::{api, components::*, invoke};
use serde::{Deserialize, Serialize};
use shared::models;
use web_sys::MouseEvent;
use yew::{html, Component, Html, Properties};

pub struct EntryView {
    entry: Option<models::Entry>,
    collections: Vec<models::Collection>,
    collections_menu_open: bool,
}

pub enum Msg {
    Update(models::Entry),
    UpdateCollections(Vec<models::Collection>),
    ToggleCollectionsMenu,
    AddToCollection { collection_id: u32 },
    None,
    Initialize { id: u32 },
}

#[derive(Properties, PartialEq)]
pub struct EntryProps {
    pub id: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTerm {
    pub id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushHistory {
    pub term_id: u32,
}

impl Component for EntryView {
    type Message = Msg;
    type Properties = EntryProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let id = ctx.props().id;
        ctx.link().send_message(Msg::Initialize { id });
        Self {
            entry: None,
            collections: vec![],
            collections_menu_open: false,
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        let id = ctx.props().id;
        ctx.link().send_message(Msg::Initialize { id });
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!(
            <>
            {
                self.entry.iter().cloned().map(|entry| {
                    let link = ctx.link().clone();
                    let onclick = move |e: MouseEvent| {
                        link.send_message(Msg::ToggleCollectionsMenu);
                        let _target = e.target().unwrap();
                    };
                    let link = ctx.link().clone();
                    html! {
                        <container>
                            <actions>
                                <start>
                                    // // Don't show pronunciation yet as file bloat is maybe not worth cost
                                    // if let Some(entry) = self.entry.clone() {
                                    //     <Pronunciation pinyin_numbers={entry.pinyin_numbers} />
                                    // }
                                </start>
                                <end>
                                    <button onclick={onclick}>
                                        {"Collect"}
                                    </button>
                                </end>
                                <Menu<models::Collection>
                                    open={self.collections_menu_open} options={self.collections.clone()}
                                    onclick={move |collection: models::Collection| {
                                    link.clone().send_message(Msg::AddToCollection { collection_id: collection.id as u32 })
                                }} />
                            </actions>
                            <Ruby entry={entry.clone()} clickable={true}></Ruby>
                            <Definition definition={entry.definition} />
                        </container>
                    }
                }).collect::<Html>()
            }
            </>
        )
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectTerm {
            pub term_id: u32,
            pub collection_id: u32,
        }

        match msg {
            Msg::Initialize { id } => {
                // get word
                ctx.link().send_future(async move {
                    let result: models::Entry = serde_wasm_bindgen::from_value({
                        invoke(
                            "get_term",
                            serde_wasm_bindgen::to_value(&GetTerm { id }).unwrap(),
                        )
                        .await
                    })
                    .unwrap();
                    Msg::Update(result)
                });

                // get collections
                ctx.link().send_future(async move {
                    let result: Vec<models::Collection> = api::collections::index().await.unwrap();
                    Msg::UpdateCollections(result)
                });

                // Add to search history
                ctx.link().send_future(async move {
                    invoke(
                        "history_create",
                        serde_wasm_bindgen::to_value(&PushHistory { term_id: id }).unwrap(),
                    )
                    .await;
                    Msg::None
                });

                true
            }
            Msg::Update(term) => {
                self.entry = Some(term);
                true
            }
            Msg::UpdateCollections(collections) => {
                self.collections = collections;
                true
            }
            Msg::ToggleCollectionsMenu => {
                self.collections_menu_open = !self.collections_menu_open;
                true
            }
            Msg::AddToCollection { collection_id } => {
                let entry_id = self.entry.clone().unwrap().id as u32;
                ctx.link().send_future(async move {
                    invoke(
                        "collections_add_term",
                        serde_wasm_bindgen::to_value(&CollectTerm {
                            term_id: entry_id,
                            collection_id,
                        })
                        .unwrap(),
                    )
                    .await;

                    Msg::None
                });
                true
            }
            Msg::None => true,
        }
    }
}

impl crate::views::View for EntryView {
    fn title(&self) -> Option<String> {
        self.entry.clone().map(|t| t.simplified)
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryArgs {
    query: String,
}
