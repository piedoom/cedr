mod bar;
mod definition;
mod example;
mod list;
mod menu;
mod patterns;
mod pronunciation;
mod ruby;
mod tabs;

pub use {
    bar::Bar,
    definition::Definition,
    //example::Example,
    list::List,
    menu::Menu,
    patterns::*,
    patterns::*,
    pronunciation::Pronunciation,
    ruby::Ruby,
    tabs::{Tab, TabGroup},
};

pub type Link = yew_router::prelude::Link<crate::Route>;
