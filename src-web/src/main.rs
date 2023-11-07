pub mod api;
mod components;
pub mod util;
mod views;

use crate::{components::*, views::*};
use shared::models;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

fn main() {
    yew::Renderer::<App>::new().render();
}

/// Interact with Tauri
pub mod commands {
    use serde::Serialize;
    use shared::InputMethod;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Query {
        pub method: InputMethod,
        pub query: String,
    }
}

// Need own struct as this framework doesn't allow sharing static strings
#[derive(Debug, Clone, PartialEq)]
pub struct Sentence {
    pub text: String,
    pub translations: Vec<String>,
}

// impl From<tatoebars::Sentence> for Sentence {
//     fn from(value: tatoebars::Sentence) -> Self {
//         Self {
//             text: value.text.to_string(),
//             translations: value
//                 .translations
//                 .into_iter()
//                 .map(ToString::to_string)
//                 .collect(),
//         }
//     }
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "primitives"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, PartialEq, yew_router::Routable)]
enum Route {
    #[at("/")]
    Search,
    #[at("/entries/:id")]
    Entry { id: u32 },
    #[at("/collections")]
    Collections,
    #[at("/collections/new")]
    CollectionNew,
    #[at("/collections/:id")]
    Collection { id: u32 },
    #[at("/history")]
    SearchHistory,
    #[at("/settings")]
    Settings,
}

pub struct App;

pub enum Msg {
    SetPreferences(models::Preferences),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let preferences = api::settings::get_preferences().await.unwrap();
            Msg::SetPreferences(preferences)
        });
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        //let navigator = ctx.link().navigator();
        html! {
            <yew_router::BrowserRouter>
                <Root/>
            </yew_router::BrowserRouter>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPreferences(preferences) => {
                let body = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .body()
                    .unwrap();
                body.set_attribute("class", &preferences.theme.to_string())
                    .unwrap();
                true
            }
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Search => html! { <SearchView/> },
        Route::Entry { id } => {
            html! { <EntryView id={id} />}
        }
        Route::SearchHistory => html! { <HistoryView/> },
        Route::Settings => html! { <SettingsView/> },
        Route::Collections => html! { <collections::Index/> },
        Route::Collection { id } => html! { <collections::Show id={id}/> },
        Route::CollectionNew => html! { <collections::New/> },
    }
}

struct Root;

impl Component for Root {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let nav = ctx.link().navigator().unwrap();
        html! {
            <app>
                <nav>
                    <TabGroup<Route>
                        initial={Route::Search}
                        onselect={move |new_value: Route| {
                        nav.push(&new_value)
                        }}>
                        <Tab<Route>
                            value={Route::Search}
                            icon={"las la-search"}>{"Search"}
                        </Tab<Route>>
                        <Tab<Route>
                            value={Route::SearchHistory}
                            icon={"las la-history"}>{"History"}
                        </Tab<Route>>
                        <Tab<Route>
                            value={Route::Collections}
                            icon={"las la-layer-group"}>{"Collections"}
                        </Tab<Route>>
                        <Tab<Route>
                            value={Route::Settings}
                            icon={"las la-sliders-h"}>{"Settings"}
                        </Tab<Route>>
                    </TabGroup<Route>>
                </nav>
                <content>
                    <yew_router::prelude::Switch<Route> render={switch} />
                </content>
            </app>
        }
    }
}
