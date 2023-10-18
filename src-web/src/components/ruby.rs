use shared::models;
use yew::prelude::*;

/// Handles the `ruby` HTML element in a more ergonomic way

pub struct Ruby;

#[derive(Properties, PartialEq)]
pub struct RubyProps {
    pub entry: models::Entry,
}

impl Component for Ruby {
    type Message = ();
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
                            .map(String::from)
                            .zip(entry.pinyin.split(' '))
                            .zip(entry.tones_u8())
                            .map(|((character, pinyin), tone)|
                            html!{
                                <>
                                    <span class={format!("tone{tone}")}>{character.clone()}</span>
                                    <rp>{"("}</rp>
                                    <rt class={format!("tone{tone}")}>{pinyin}</rt>
                                    <rp>{")"}</rp>
                                </>
                            }
                        ).collect::<Html>()
                    }
                </ruby>
            </div>
        }
    }
}
