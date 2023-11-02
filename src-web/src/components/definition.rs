use yew::prelude::*;

pub struct Definition;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub definition: String,
    /// Maximum number of definitions to show
    #[prop_or_default]
    pub limit: Option<usize>,
}

impl Component for Definition {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let definition = ctx.props().definition.clone();
        let mut split: Vec<&str> = definition.split('\n').collect();
        if let Some(limit) = ctx.props().limit {
            split.truncate(limit)
        };
        split
            .into_iter()
            .map(|def| {
                html! { <p> { def } </p> }
            })
            .collect::<Html>()
    }
}
