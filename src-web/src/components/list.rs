use std::marker::PhantomData;

use yew::prelude::*;

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
        let ListProps { items, render } = ctx.props();

        html! {
            <list>
                { items
                    .iter()
                    .map(|item| {
                        html! {
                            <>
                                { render.emit(item.clone()) }
                                <hr/>
                            </>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </list>
        }
    }
}
//
