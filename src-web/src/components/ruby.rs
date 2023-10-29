use shared::models;
use web_sys::HtmlSpanElement;
use yew::prelude::*;

/// Handles the `ruby` HTML element in a more ergonomic way

pub struct Ruby;

#[derive(Properties, PartialEq)]
pub struct RubyProps {
    pub entry: models::Entry,
}

pub enum Msg {
    CharacterPopoverShow {
        event: MouseEvent,
        traditional: String,
    },
    CharacterPopoverHide {
        event: MouseEvent,
        traditional: String,
    },
}

impl Component for Ruby {
    type Message = Msg;
    type Properties = RubyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let entry = ctx.props().entry.clone();
        html! {
            // Wrap in a div to fix the issue where the box model doesn't account for the pronunciation
            <div>
                <ruby>
                    {

                        entry.simplified.chars()
                            .zip(entry.traditional.chars())
                            .zip(entry.pinyin.split(' '))
                            .zip(entry.tones_u8())
                            .map(|(((character,traditional), pinyin), tone)| {

                            let link = ctx.link().clone();
                            let show_char = move |event: MouseEvent| { link.send_message(Msg::CharacterPopoverShow { event, traditional: traditional.to_string() }) };
                            let link = ctx.link().clone();
                            let hide_char = move |event: MouseEvent| { link.send_message(Msg::CharacterPopoverHide { event, traditional: traditional.to_string() }) };

                            html!{
                                <>
                                    <span onmouseleave={hide_char} onmouseover={show_char} class={format!("tone{tone}")}>{character.clone()}</span>
                                    <rp>{"("}</rp>
                                    <rt class={format!("tone{tone}")}>{pinyin}</rt>
                                    <rp>{")"}</rp>
                                </>
                            }
                        }).collect::<Html>()
                    }
                </ruby>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CharacterPopoverShow { event, traditional } => {
                //
                // let target: HtmlSpanElement = event.target_unchecked_into();
                // ctx.link().get_parent().
                // target.
                true
            }
            Msg::CharacterPopoverHide { event, traditional } => {
                //
                true
            }
        }
    }
}
