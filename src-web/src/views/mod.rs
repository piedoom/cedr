pub mod collections;
pub(crate) mod entry;
mod history;
mod search;
mod settings;

pub use entry::EntryView;
pub use history::HistoryView;
pub use search::SearchView;
pub use settings::SettingsView;

pub trait View {
    fn title(&self) -> Option<String>;
}
