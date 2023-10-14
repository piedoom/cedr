use yew::prelude::*;

use crate::Sentence;

use super::*;
pub struct Examples;

#[derive(Properties, PartialEq)]
pub struct ExamplesProps {
    pub examples: Vec<Sentence>,
}

impl Component for Examples {
    type Message = ();
    type Properties = ExamplesProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props()
            .examples
            .clone()
            .iter()
            .map(|sentence| {
                html! {
                    <>
                        <p>{&sentence.text}</p>
                        {sentence.translations.iter().map(|translation| {
                            html! {
                                <p>{translation}</p>
                            }
                        }).collect::<Html>()}
                    </>
                }
            })
            .collect::<Html>()
    }
}
