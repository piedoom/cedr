use yew::prelude::*;

use crate::{components, views::View};

pub struct SettingsView;

impl Component for SettingsView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html!(
            <components::Bar title={self.title()} ></components::Bar>
        )
    }
}

impl crate::views::View for SettingsView {
    fn title(&self) -> Option<String> {
        Some("Settings".into())
    }
}
