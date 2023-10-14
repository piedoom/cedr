//! Generic page level top bar with the title and actions

use yew::prelude::*;

pub struct Bar;

impl Component for Bar {
    type Message = ();

    type Properties = BarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let BarProps {
            title,
            back_button,
            children,
        } = ctx.props();
        html! {
            <topbar>
                {
                    back_button.then(|| {
                        html! {
                            <button variant="secondary" onclick={|_e: MouseEvent| {
                                web_sys::window().unwrap().history().unwrap().back().ok();
                            }}>
                                <i class="las la-angle-left"></i>{"Back"}
                            </button>
                        }
                    }).unwrap_or_default()
                }

                <div style="flex: 1">
                {
                    title.clone().map(|title| {
                        html! {
                            <h3>{title}</h3>
                        }
                    })
                }
                </div>

                { children }
            </topbar>
        }
    }
}

#[derive(Properties, PartialEq, Default)]
pub struct BarProps {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub back_button: bool,
    #[prop_or_default]
    pub children: Children,
}
