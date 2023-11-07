use shared::models::{self, Theme};
use yew::prelude::*;

use crate::api;

pub struct SettingsView {
    preferences: Option<models::Preferences>,
}

pub enum Msg {
    None,
    SetPreferences(models::Preferences),
}

impl From<()> for Msg {
    fn from(_: ()) -> Self {
        Msg::None
    }
}

impl Component for SettingsView {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async {
            let preferences = api::settings::get_preferences().await.unwrap();
            Msg::SetPreferences(preferences)
        });
        Self { preferences: None }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link().clone();
        let update_dictionary = move |_| {
            link.send_future(async {
                api::settings::update_cedict().await;
            });
        };

        let link = ctx.link().clone();
        let import_collections = move |_| {
            link.send_future(async {
                api::settings::export_collections().await;
            });
        };

        let link = ctx.link().clone();
        let export_collections = move |_| {
            link.send_future(async {
                api::settings::import_collections().await;
            });
        };

        let link = ctx.link().clone();
        let preferences: models::Preferences = self.preferences.clone().unwrap_or_default();
        let set_theme = link.callback(move |theme: Theme| {
            let mut preferences = preferences.clone();
            let body = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap();
            body.set_attribute("class", &theme.to_string()).unwrap();
            preferences.theme = theme;
            Msg::SetPreferences(preferences)
        });
        let st2 = set_theme.clone();
        let st3 = set_theme.clone();

        html!(
            <container>
                <h2>{"Dictionary"}</h2>
                <button onclick={update_dictionary}>{"Update dictionary"}</button>
                <hr/>

                <h2>{"Collections"}</h2>
                <button onclick={import_collections}>{"Import"}</button>
                <button onclick={export_collections}>{"Export"}</button>

                <h2>{"Theme"}</h2>
                // TODO: Enable auto
                // <button onclick={move |_| set_theme.emit(Theme::Auto)}>{"Auto theme"}</button>
                <button onclick={move |_| st2.emit(Theme::Light)}>{"Light theme"}</button>
                <button onclick={move |_| st3.emit(Theme::Dark)}>{"Dark theme"}</button>
                <hr/>
            </container>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::SetPreferences(preferences) => {
                self.preferences = Some(preferences.clone());
                ctx.link().send_future(async move {
                    api::settings::set_preferences(preferences.clone()).await
                });
                true
            }
        }
    }
}
