use shared::models;
use yew::prelude::*;

use crate::{api, components::*, util::Size, View};

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
            let results = api::collections::index().await.unwrap();
            CollectionsMsg::Update(results)
        });
        Self {
            collections: Default::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <Split<models::Collection>
                items={self.collections.clone()}
                width={Size::Em(16f32)}
                render_list={|collection: models::Collection|
                    html! {
                       <h3>{collection.name}</h3>
                    }
                }
                render_split={|collection: models::Collection|
                    html! {
                        <super::Show id={collection.id as u32} />
                    }
                }
            />
        </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollectionsMsg::Update(collections) => {
                self.collections = collections;
                true
            }
        }
    }
}
