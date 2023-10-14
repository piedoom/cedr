use shared::models;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::{components::*, invoke, Route, View};

pub struct Index {
    pub collections: Vec<models::Collection>,
}

impl View for Index {
    fn title(&self) -> Option<String> {
        Some("Collections".to_string())
    }
}

pub enum CollectionsMsg {
    Update(Vec<models::Collection>),
}

impl Component for Index {
    type Message = CollectionsMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let results: Vec<models::Collection> =
                serde_wasm_bindgen::from_value(invoke("collections_index", JsValue::null()).await)
                    .unwrap();

            CollectionsMsg::Update(results)
        });
        Self {
            collections: Default::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            <Bar title={self.title()}>
                <Link to={Route::CollectionNew}><button>{"New"}</button></Link>
            </Bar>
            <List<models::Collection> items={self.collections.clone()} render={|collection: models::Collection|
                //
                {
                    html! {
                        <Link to={Route::Collection { id: collection.id as u32 }}>
                            <h3>{collection.name}</h3>
                        </Link>
                    }
                }
            } />
        </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollectionsMsg::Update(collections) => {
                self.collections = collections;
                true
            }
        }
    }
}
