use crate::components::settings::Settings;
use gloo_storage::Storage;
use leptos::*;

/*
pub fn store_pages(pages: &Vec<Vec<Vec<Props>>>) {
    if let Err(e) = gloo_storage::LocalStorage::set("pages", pages) {
        logging::log!("could not store pages: {e:?}");
    }
}
pub fn get_pages() -> Option<Vec<Vec<Vec<Props>>>> {
    gloo_storage::LocalStorage::get("pages").ok()
}
*/

pub fn store_settings(settings: &Settings) {
    if let Err(e) = gloo_storage::LocalStorage::set("settings", settings) {
        logging::log!("could not store settings: {e:?}");
    }
}
pub fn get_settings() -> Option<Settings> {
    gloo_storage::LocalStorage::get("settings").ok()
}
