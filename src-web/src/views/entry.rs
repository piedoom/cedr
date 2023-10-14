use crate::{components::*, invoke, views::View};
use serde::{Deserialize, Serialize};
use shared::models;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;
use yew::{html, Component, Html, Properties};

pub struct EntryView {
    entry: Option<models::Entry>,
    collections: Vec<models::Collection>,
    collections_menu_open: bool,
}

pub enum Message {
    Update(models::Entry),
    UpdateCollections(Vec<models::Collection>),
    ToggleCollectionsMenu,
    AddToCollection { collection_id: u32 },
    None,
}

impl Component for EntryView {
    type Message = Message;
    type Properties = EntryProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTerm {
            pub id: u32,
            pub include_sentences: bool,
        }
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct PushHistory {
            pub term_id: u32,
        }

        let id = ctx.props().id;
        // get word
        ctx.link().send_future(async move {
            let result: models::Entry = serde_wasm_bindgen::from_value({
                invoke(
                    "get_term",
                    serde_wasm_bindgen::to_value(&GetTerm {
                        id,
                        include_sentences: true,
                    })
                    .unwrap(),
                )
                .await
            })
            .unwrap();
            Message::Update(result)
        });

        // get collections
        ctx.link().send_future(async move {
            let result: Vec<models::Collection> = serde_wasm_bindgen::from_value({
                invoke("collections_index", JsValue::null()).await
            })
            .unwrap();
            Message::UpdateCollections(result)
        });

        // Add to search history
        ctx.link().send_future(async move {
            invoke(
                "history_create",
                serde_wasm_bindgen::to_value(&PushHistory { term_id: id }).unwrap(),
            )
            .await;
            Message::None
        });

        Self {
            entry: None,
            collections: Default::default(),
            collections_menu_open: false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link().clone();
        let onclick = move |e: MouseEvent| {
            link.send_message(Message::ToggleCollectionsMenu);
            let target = e.target().unwrap();
        };
        let link = ctx.link().clone();

        html!(
            <>
                <Bar back_button={true} title={self.title()} >
                    <button onclick={onclick}>
                        {"Collect"}
                    </button>
                    <Menu<models::Collection>
                        open={self.collections_menu_open} options={self.collections.clone()}
                        onclick={move |collection: models::Collection| {
                        link.clone().send_message(Message::AddToCollection { collection_id: collection.id as u32 })
                    }} />
                </Bar>
                {
                    self.entry.iter().cloned().map(|entry| {
                        html! {
                            <container>
                                // <components::Ruby simplified={self.0.clone().map(|x| x.simplified).unwrap_or_default()} pinyin={self.0.clone().map(|x| x.pinyin).unwrap_or_default()} tones={self.0.clone().map(|x| x.tones_u8()).unwrap_or_default()}></components::Ruby>
                                <Ruby term={entry.term}></Ruby>
                                <Definitions definitions={entry.definitions}/>
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
            Message::Update(term) => {
                self.entry = Some(term);
                true
            }
            Message::UpdateCollections(collections) => {
                self.collections = collections;
                true
            }
            Message::ToggleCollectionsMenu => {
                self.collections_menu_open = !self.collections_menu_open;
                true
            }
            Message::AddToCollection { collection_id } => {
                let term_id = self.entry.clone().unwrap().term.id as u32;
                ctx.link().send_future(async move {
                    invoke(
                        "collections_add_term",
                        serde_wasm_bindgen::to_value(&CollectTerm {
                            term_id,
                            collection_id,
                        })
                        .unwrap(),
                    )
                    .await;

                    Message::None
                });
                true
            }
            Message::None => false,
        }
    }
}

impl crate::views::View for EntryView {
    fn title(&self) -> Option<String> {
        self.entry.clone().map(|t| t.term.simplified)
    }
}

#[derive(Properties, PartialEq)]
pub struct EntryProps {
    pub id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct QueryArgs {
    query: String,
}
