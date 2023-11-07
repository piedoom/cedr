use std::marker::PhantomData;

use yew::prelude::*;

pub struct Tab<T>
where
    T: PartialEq,
{
    _phantom_data: PhantomData<T>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct TabProps<T>
where
    T: PartialEq,
{
    pub value: T,
    #[prop_or_default]
    pub icon: Option<String>,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

impl<T> Component for Tab<T>
where
    T: PartialEq + 'static,
{
    type Message = ();

    type Properties = TabProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _phantom_data: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <tab class={ctx.props().class.clone()}>
                {
                    ctx.props().icon.clone().map(|icon| {
                        html! { <i class={icon} /> }
                    })
                }
                { ctx.props().children.iter().collect::<Html>() }
            </tab>
        }
    }
}

pub struct TabGroup<T>
where
    T: PartialEq,
{
    pub value: T,
}

#[derive(Properties, PartialEq)]
pub struct TabGroupProps<T>
where
    T: PartialEq + Clone + 'static,
{
    pub initial: T,
    pub onselect: Callback<T, ()>,
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab<T>>,
}

pub enum Msg<T> {
    Select(T),
}

impl<T> Component for TabGroup<T>
where
    T: PartialEq + Clone + 'static,
{
    type Message = Msg<T>;

    type Properties = TabGroupProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: ctx.props().initial.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <tabs>
                <horizontal>
                    { ctx.props().children.iter().map(|child| {
                        let props = child.props.clone();
                        let link = ctx.link().clone();
                        let class = if props.value == self.value { "active" } else { "" };
                        html! {
                            <div onclick={move |_| link.send_message({
                                Msg::Select(props.value.clone())
                            })}>
                                <Tab<T> class={class} value={child.props.clone().value.clone()} icon={child.props.clone().icon.clone()}>
                                    {child.props.clone().children.clone()}
                                </Tab<T>>
                            </div>
                        }
                    }).collect::<Html>() }
                </horizontal>
            </tabs>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(value) => {
                self.value = value.clone();
                ctx.props().onselect.emit(value);
                true
            }
        }
    }
}
