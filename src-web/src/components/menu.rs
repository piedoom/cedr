use std::fmt::Display;
use std::marker::PhantomData;

use yew::prelude::*;

pub struct Menu<T>
where
    T: Display,
{
    _phantom_data: PhantomData<T>,
    pub open: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Display + PartialEq,
{
    pub open: bool,
    pub options: Vec<T>,
    pub onclick: Callback<T, ()>,
}

pub enum Msg<T> {
    Select(T),
}

impl<T> Component for Menu<T>
where
    T: Display + PartialEq + Clone + 'static,
{
    type Message = Msg<T>;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _phantom_data: PhantomData,
            open: ctx.props().open,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.open = ctx.props().open;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let options: Vec<T> = ctx.props().options.clone();
        html! {
            <>
            if self.open {
                <menucontainer>
                <menu>
                    {
                        options.into_iter().map(move |option| {
                            let link = ctx.link().clone();
                            let option_clone = option.clone();
                            html! {
                                <menuitem onclick={move |_| link.send_message(Msg::Select(option.clone()))}>
                                    {format!("{}", option_clone)}
                                </menuitem>
                            }
                        }).collect::<Html>()
                    }
                </menu>
                </menucontainer>
            }
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(option) => {
                ctx.props().onclick.emit(option);
                self.open = false;
                true
            }
        }
    }
}
