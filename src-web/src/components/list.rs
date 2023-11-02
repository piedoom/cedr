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
        let ListProps {
            items,
            render,
            route,
        } = ctx.props();

        html! {
            <list>
                { items
                    .iter()
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
                            None => html! { <listitem> {list_item} </listitem> }
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </list>
        }
    }
}
//
