use std::marker::PhantomData;

use yew::prelude::*;

use super::Link;

pub struct List<T>
where
    T: PartialEq,
{
    _phantom_data: PhantomData<T>,
}

#[derive(Properties, PartialEq)]
pub struct ListProps<T>
where
    T: PartialEq + Clone + 'static,
{
    pub items: Vec<T>,
    pub render: Callback<T, Html>,
    #[prop_or_default]
    pub route: Option<Callback<T, crate::Route>>,
    #[prop_or_default]
    pub onclick: Option<Callback<(MouseEvent, T), ()>>,
}

impl<T> Component for List<T>
where
    T: PartialEq + Clone + 'static,
{
    type Message = ();

    type Properties = ListProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _phantom_data: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items = ctx.props().items.clone();
        let render = ctx.props().render.clone();
        let route = ctx.props().route.clone();
        let onclick = ctx.props().onclick.clone();

        html! {
            <list>
                { items
                    .into_iter()
                    .map(|item| {
                        let list_item = render.emit(item.clone());
                        match route.clone() {
                            Some(route) => {
                                html! {
                                    <Link to={route.emit(item.clone())}>
                                        <listitem>
                                            {list_item}
                                        </listitem>
                                    </Link>
                                }
                            }
                            None => {
                                let onclick = onclick.clone();
                                // Only show as clickable if set
                                let styles = if onclick.is_some() {
                                    "cursor: pointer;"
                                } else { "" };
                                let item_onclick = move |ev: MouseEvent, item: T| {
                                    if let Some(onclick) = onclick.clone() {
                                        onclick.emit((ev, item));
                                    }
                                };

                                html! {
                                    <listitem style={styles} onclick={move |ev| { item_onclick(ev, item.clone()); } }>
                                        {list_item}
                                    </listitem>
                                }
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </list>
        }
    }
}
//
