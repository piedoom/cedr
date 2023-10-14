use shared::models;
use yew::prelude::*;

pub use definitions::Definitions;

#[derive(Properties, PartialEq)]
pub struct TermProps {
    pub term: models::Term,
}

/// Handles english definitions display
pub mod definitions {
    use super::*;
    pub struct Definitions;

    #[derive(Properties, PartialEq)]
    pub struct EntriesProps {
        pub definitions: Vec<models::Definition>,
    }

    impl Component for Definitions {
        type Message = ();
        type Properties = EntriesProps;

        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            ctx.props()
                .definitions
                .iter()
                .map(|definition| {
                    html! {
                        <p>{&definition.definition}</p>
                    }
                })
                .collect::<Html>()
        }
    }
}
