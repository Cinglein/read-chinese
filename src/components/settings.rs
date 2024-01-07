use crate::logic::storage::store_settings;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub pinyin: PinyinSettings,
    pub font: Font,
    pub words_page: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            pinyin: Default::default(),
            font: Default::default(),
            words_page: 512,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinyinSettings {
    #[default]
    Display,
    Hide,
    SpacedRep,
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Font {
    Sans,
    #[default]
    Serif,
    Cursive,
}

#[component]
pub fn Settings(
    settings: ReadSignal<Settings>,
    set_settings: WriteSignal<Settings>,
) -> impl IntoView {
    let fontbg = move |font: Font| {
        if font == settings().font {
            "border-slate-300 border-b-4 border-l-2 shadow-lg bg-sky-100"
        } else {
            "border-slate-300 border-b-4 border-l-2 shadow-lg bg-slate-200"
        }
    };
    let pinyinbg = move |pinyin: PinyinSettings| {
        if pinyin == settings().pinyin {
            "border-slate-300 border-b-4 border-l-2 shadow-lg bg-rose-100"
        } else {
            "border-slate-300 border-b-4 border-l-2 shadow-lg bg-slate-200"
        }
    };
    view! {
        <div class="flex flex-col flex-nowrap p-1 lg:w-[40rem] w-screen">
            <div class="grid grid-cols-3 gap-0">
                <div
                    class=move || format!(
                        "flex items-center justify-center m-1 p-2 lg:h-10 h-8 \
                        cursor-pointer lg:lg:text-md text-sm text-sm rounded hover:bg-sky-50 {}",
                        fontbg(Font::Serif),
                    )
                    on:click=move |_| set_settings.update(|old| {
                        old.font = Font::Serif;
                        store_settings(old);
                    })
                >Serif</div>
                <div
                    class=move || format!(
                        "flex items-center justify-center m-1 p-2 lg:h-10 h-8 \
                        cursor-pointer lg:text-md text-sm rounded hover:bg-sky-50 {}",
                        fontbg(Font::Sans),
                    )
                    on:click=move |_| set_settings.update(|old| {
                        old.font = Font::Sans;
                        store_settings(old);
                    })
                >Sans-Serif</div>
                <div
                    class=move || format!(
                        "flex items-center justify-center m-1 p-2 lg:h-10 h-8 \
                        cursor-pointer lg:text-md text-sm rounded hover:bg-sky-50 {}",
                        fontbg(Font::Cursive),
                    )
                    on:click=move |_| set_settings.update(|old| {
                        old.font = Font::Cursive;
                        store_settings(old);
                    })
                >Cursive</div>
            </div>
            <div class="grid grid-cols-3 gap-0">
                <div
                    class=move || format!(
                        "flex items-center justify-center m-1 p-2 lg:h-10 h-8 \
                        cursor-pointer lg:text-md text-sm rounded hover:bg-sky-50 {}",
                        pinyinbg(PinyinSettings::Display),
                    )
                    on:click=move |_| set_settings.update(|old| {
                        old.pinyin = PinyinSettings::Display;
                        store_settings(old);
                    })
                >Display Pinyin</div>
                <div
                    class=move || format!(
                        "flex items-center justify-center m-1 p-2 lg:h-10 h-8 \
                        cursor-pointer lg:text-md text-sm rounded hover:bg-sky-50 {}",
                        pinyinbg(PinyinSettings::Hide),
                    )
                    on:click=move |_| set_settings.update(|old| {
                        old.pinyin = PinyinSettings::Hide;
                        store_settings(old);
                    })
                >Hide Pinyin</div>
                <div
                    class=move || format!(
                        "flex items-center justify-center m-1 p-2 lg:h-10 h-8 \
                        cursor-pointer lg:text-md text-sm text-center rounded hover:bg-sky-50 {}",
                        pinyinbg(PinyinSettings::SpacedRep),
                    )
                    on:click=move |_| set_settings.update(|old| {
                        old.pinyin = PinyinSettings::SpacedRep;
                        store_settings(old);
                    })
                >Spaced Rep</div>
            </div>
        </div>
    }
}
