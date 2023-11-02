use shared::models;
use yew::prelude::*;

use crate::api;

pub struct QuizView;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// If none, will review all collection items
    #[prop_or_default]
    pub collection_ids: Vec<u32>,
}

pub enum Msg {
    Update(Vec<models::CollectionWithEntries>),
}

impl Component for QuizView {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // get all entries in the collection
        let collection_ids = ctx.props().collection_ids.clone();
        ctx.link().send_future(async move {
            let mut results = vec![];
            // Get specified collections or all collections if no collections are specified
            let collection_ids = match collection_ids.is_empty() {
                true => api::collections::index()
                    .await
                    .unwrap()
                    .into_iter()
                    .map(|x| x.id as u32)
                    .collect(),
                false => collection_ids,
            };

            // Get all entries in specified collections
            for id in collection_ids {
                results.push(api::collections::get(id).await.unwrap())
            }

            // Update state of the component
            Msg::Update(results)
        });
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <container>

                </container>
            </>
        }
    }
}
