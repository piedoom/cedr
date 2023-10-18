mod components;
mod views;

use crate::{components::*, views::*};
use hex_color::HexColor;
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

impl From<tatoebars::Sentence> for Sentence {
    fn from(value: tatoebars::Sentence) -> Self {
        Self {
            text: value.text.to_string(),
            translations: value
                .translations
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub(crate) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
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

pub struct Settings {
    pub theme: Theme,
}

pub struct Theme {
    pub background: HexColor,
    pub tones: ToneTheme,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme {
                background: HexColor::parse("#1B1B1A").unwrap(),
                tones: ToneTheme::default(),
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct ToneTheme([HexColor; 5]);

impl ToneTheme {
    pub fn get(&self, tone: &u8) -> &HexColor {
        let index = (*tone) as usize - 1;
        self.0
            .get(index)
            .unwrap_or_else(|| panic!("No tone found for index {}", index))
    }
}

impl Default for ToneTheme {
    fn default() -> Self {
        [
            HexColor::parse("#cc2400").unwrap(),
            HexColor::parse("#4e9d01").unwrap(),
            HexColor::parse("#0092e6").unwrap(),
            HexColor::parse("#b700c7").unwrap(),
            HexColor::parse("#6e6e6e").unwrap(),
        ]
        .into()
    }
}

impl From<[HexColor; 5]> for ToneTheme {
    fn from(value: [HexColor; 5]) -> Self {
        Self(value)
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        //let navigator = ctx.link().navigator();
        html! {
            <yew_router::BrowserRouter>
                <Root/>
            </yew_router::BrowserRouter>
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
            <horizontal>
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
            </horizontal>
        }
    }
}
