mod bar;
mod example;
mod list;
mod menu;
mod ruby;
mod tabs;

pub use {
    bar::Bar,
    //example::Example,
    list::List,
    menu::Menu,
    ruby::Ruby,
    tabs::{Tab, TabGroup},
};

pub type Link = yew_router::prelude::Link<crate::Route>;
