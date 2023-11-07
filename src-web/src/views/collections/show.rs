use shared::models;
use yew::prelude::*;

use crate::{api, components::*, util::Size};

pub struct Show(pub Option<models::CollectionWithEntries>);

pub enum CollectionMsg {
    Update(models::CollectionWithEntries),
}

#[derive(Properties, PartialEq)]
pub struct CollectionProps {
    pub id: u32,
}

impl Component for Show {
    type Message = CollectionMsg;

    type Properties = CollectionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id;
        ctx.link().send_future(async move {
            let result = api::collections::get(id).await.unwrap();
            CollectionMsg::Update(result)
        });
        Self(Default::default())
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            if self.0.is_some() {
                <Split<models::Entry>
                    items={self.0.clone().unwrap().entries}
                    width={Size::Em(16f32)}
                    render_list={|entry: models::Entry|
                        html! {
                            <>
                                <Ruby entry={entry.clone()} />
                                <Definition definition={entry.definition} limit={1} />
                            </>
                        }
                    }
                    render_split={|collection: models::Entry|
                        html! {
                            <crate::views::EntryView id={collection.id as u32} />
                        }
                    }
                />
            }
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
