use shared::models;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, FormData, HtmlFormElement};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{components::*, invoke, Route, View};

pub struct New;

impl View for New {
    fn title(&self) -> Option<String> {
        Some("New collection".to_string())
    }
}

#[derive(serde::Serialize)]
pub struct CollectionCreateArgs {
    name: String,
}

pub enum CollectionNewMsg {
    Create { name: String },
    Redirect { id: u32 },
}

impl Component for New {
    type Message = CollectionNewMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let form_onsubmit = {
            let link = ctx.link().clone();
            Callback::from(move |event: SubmitEvent| {
                let target: EventTarget = event.target().unwrap();
                let form = target.dyn_into::<HtmlFormElement>().unwrap();

                let form_data = FormData::new_with_form(&form).unwrap();
                let name = form_data.get("name").as_string().unwrap();
                link.send_message(Self::Message::Create { name })
            })
        };

        html! {
            <>
                <Bar title={self.title()}/>
                <container>
                    <form onsubmit={form_onsubmit}>
                        <label>{"Name"}</label>
                        <input type="text" name="name"/>
                        <input type="submit" value="Create"/>
                    </form>
                </container>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollectionNewMsg::Create { name } => {
                let link = ctx.link().clone();
                link.clone().send_future(async move {
                    let id: u32 = serde_wasm_bindgen::from_value(
                        invoke(
                            "collections_create",
                            serde_wasm_bindgen::to_value(&CollectionCreateArgs { name }).unwrap(),
                        )
                        .await,
                    )
                    .unwrap();

                    Self::Message::Redirect { id }
                });
                false
            }
            CollectionNewMsg::Redirect { id } => {
                ctx.link()
                    .navigator()
                    .unwrap()
                    .push(&Route::Collection { id });
                ctx.link().navigator().unwrap().forward();
                true
            }
        }
    }
}
