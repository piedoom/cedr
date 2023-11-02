use shared::models;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{api, Route};

/// Handles the `ruby` HTML element in a more ergonomic way

pub struct Ruby;

#[derive(Properties, PartialEq)]
pub struct RubyProps {
    pub entry: models::Entry,
}

pub enum Msg {
    Navigate { traditional: char },
    ExecuteNavigate { id: u32 },
}

impl Component for Ruby {
    type Message = Msg;
    type Properties = RubyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let entry = ctx.props().entry.clone();
        let chars = entry.pinyin.split(' ').count();
        html! {
            <horizontal class="end sm">
                <rb style={format!("grid-template-columns: repeat({chars}, 1fr);")}>
                    {
                        entry.pinyin.split(' ').into_iter().map(|py| html! {
                            <pinyin>{py}</pinyin>
                        }).collect::<Html>()
                    }
                    {
                        entry.simplified.chars()
                            .zip(entry.traditional.chars())
                            .zip(entry.tones_u8())
                            .map(|((ch, traditional), tone)| {
                                let link = ctx.link().clone();
                                {
                                    html! {
                                        <hanzi class={format!("tone{tone}")} onclick={move |_| link.send_message(Msg::Navigate { traditional })} >{ch}</hanzi>
                                    // onmouseleave={hide_char} onmouseover={show_char}
                                    }
                                }
                            }).collect::<Html>()
                    }
                    if entry.traditional != entry.simplified {
                        {
                            entry.traditional.chars()
                                .zip(entry.simplified.chars())
                                .zip(entry.tones_u8())
                                .map(|((traditional, simplified), tone)| {
                                    let link = ctx.link().clone();
                                    {
                                        html! {
                                            if traditional != simplified {
                                                <traditional onclick={move |_| link.send_message(Msg::Navigate { traditional })} >{traditional}</traditional>
                                            } else {
                                                <div></div>
                                            }
                                        // onmouseleave={hide_char} onmouseover={show_char}
                                        }
                                    }
                                }).collect::<Html>()
                        }
                    }
                </rb>
            </horizontal>

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Navigate { traditional } => {
                // If trying to navigate to the same character, prevent this behavior.
                // TODO: Indicate this from the UI first
                if ctx.props().entry.traditional != traditional.to_string() {
                    ctx.link().send_future(async move {
                        let ch = api::entries::get_by_traditional(traditional).await.unwrap();
                        Msg::ExecuteNavigate { id: ch.id as u32 }
                    });
                }
                true
            }
            Msg::ExecuteNavigate { id } => {
                ctx.link().navigator().unwrap().push(&Route::Entry { id });
                true
            }
        }
    }
}
