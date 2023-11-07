use crate::util::{Direction, Size};
use yew::prelude::*;

use crate::components::*;

pub struct Split<T>
where
    T: PartialEq,
{
    selection: Option<T>,
}

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Clone + PartialEq,
{
    pub items: Vec<T>,
    /// Render the current item T
    pub render_split: Callback<T, Html>,
    #[prop_or_default]
    pub direction: Direction,
    pub render_list: Callback<T, Html>,
    /// Flex property of the leading-side list
    #[prop_or_default]
    pub width: Option<Size>,
}

pub enum Msg<T> {
    Select(Option<T>),
}

impl<T> Component for Split<T>
where
    T: PartialEq + Clone + 'static,
{
    type Message = Msg<T>;

    type Properties = Props<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { selection: None }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            items,
            render_split,
            direction,
            render_list,
            width,
        } = ctx.props();

        let onclick = ctx.link().callback(|(_, item)| Msg::Select(Some(item)));
        let selected = self.selection.clone();
        let mut width = width.as_ref();

        // If nothing is opened yet, do not add any width styles
        if selected.is_none() {
            width = None;
        }

        let width = width
            .map(|width| format!("width: {width}"))
            .unwrap_or("flex: 1".into());

        let content = html! {
            <>
                <div style={width}>
                    <List<T> items={items.clone()} render={render_list} {onclick} />
                </div>
                if selected.is_some() {
                    <div style={"flex: 1"}>
                        {render_split.emit(selected.unwrap())}
                    </div>
                }
            </>
        };
        match direction {
            Direction::Vertical => html! { <vertical style="height: 100%">{content}</vertical> },
            Direction::Horizontal => {
                html! { <horizontal style="width: 100%">{content}</horizontal> }
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(selection) => {
                if selection == self.selection {
                    // nothing changed
                    false
                } else {
                    self.selection = selection.clone();
                    true
                }
            }
        }
    }
}
