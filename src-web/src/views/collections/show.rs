use shared::models;
use yew::prelude::*;

use crate::{components::*, invoke, views::View, Route};

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            if self.0.is_some() {
            <Bar back_button={true} title={self.title()}/>
            <List<models::Entry>
                items={self.0.clone().unwrap().entries}
                render={|entry: models::Entry| {
                    html! {
                        <Link to={Route::Entry { id: entry.id as u32 }}>
                            <Ruby entry={entry.clone()} />
                            <p> { entry.definition } </p>
                        </Link>
                    }
                }}>
            </List<models::Entry>>
            }
        </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollectionMsg::Update(collection) => {
                self.0 = Some(collection);
                true
            }
        }
    }
}

impl crate::View for Show {
    fn title(&self) -> Option<String> {
        self.0.clone().map(|t| t.collection.name)
    }
}
