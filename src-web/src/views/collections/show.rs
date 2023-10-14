use shared::models;
use yew::prelude::*;

use crate::{components::*, invoke, Route};

pub struct Show(pub Option<models::CollectionWithEntries>);

pub enum CollectionMsg {
    Update(models::CollectionWithEntries),
}

#[derive(Properties, PartialEq)]
pub struct CollectionProps {
    pub id: u32,
}

#[derive(serde::Serialize)]
pub struct CollectionArgs {
    id: u32,
}

impl Component for Show {
    type Message = CollectionMsg;

    type Properties = CollectionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id;
        ctx.link().send_future(async move {
            let result: models::CollectionWithEntries = serde_wasm_bindgen::from_value(
                invoke(
                    "collections_get",
                    serde_wasm_bindgen::to_value(&CollectionArgs { id }).unwrap(),
                )
                .await,
            )
            .unwrap();

            CollectionMsg::Update(result)
        });
        Self(Default::default())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            if self.0.is_some() {
            <List<models::Entry>
                items={self.0.clone().unwrap().entries}
                render={|entry: models::Entry| {
                    html! {
                        <Link to={Route::Entry { id: entry.term.id as u32 }}>
                            <Ruby term={entry.term} />
                            <Definitions definitions={entry.definitions} />
                        </Link>
                    }
                }}>
            </List<models::Entry>>
            }
        </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollectionMsg::Update(collection) => {
                self.0 = Some(collection);
                true
            }
        }
    }
}
