use crate::{components::settings::*, logic::dict};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Props {
    character: String,
    reading: Option<dict::Reading<String>>,
    spacedrep: bool,
}

impl Props {
    pub fn new(character: &str, spacedrep: bool) -> Self {
        let character = character.to_string();
        let reading = dict::DICT.get(&character).map(|r| r.into());
        Self {
            character,
            reading,
            spacedrep,
        }
    }
    fn view(
        &self,
        settings: ReadSignal<Settings>,
        is_clicked: ReadSignal<bool>,
        on_click: WriteSignal<bool>,
    ) -> impl IntoView {
        let font_character = move || match settings().font {
            Font::Sans => "font-sans",
            Font::Serif => "font-serif",
            Font::Cursive => "font-cursive",
        };
        let font_reading = move || match settings().font {
            Font::Sans => "font-sans",
            Font::Serif => "font-serif",
            Font::Cursive => "font-serif",
        };
        let character = self.character.clone();
        if let Some(reading) = &self.reading {
            let def = reading.def.clone();
            let reading = reading.reading.clone();
            let spacedrep = self.spacedrep.clone();
            view! {
                <div class="m-2 lg:h-24 h-14">
                    <Show
                        when=move || is_clicked()
                    >
                        <div
                            on:click=move |_| on_click(false)
                            class="m-auto p-2 w-40 break-words text-center rounded-md bg-gray-700 \
                                absolute lg:text-xl text-sm text-gray-200 font-serif"
                        ><span class="relative">{&def}</span></div>
                    </Show>
                    <div
                        class=move || format!(
                            "lg:text-7xl text-4xl text-center text-gray-900 {}",
                            font_character(),
                        )
                        on:click=move |_| { on_click(true) }
                    >{character}</div>
                    <Show
                        when=move || match settings().pinyin {
                            PinyinSettings::Display => true,
                            PinyinSettings::Hide => false,
                            PinyinSettings::SpacedRep => spacedrep,
                        }
                    >
                        <div
                            class=move || format!(
                                "lg:text-3xl text-xl text-center text-gray-300 {}",
                                font_reading(),
                            )
                        >{&reading}</div>
                    </Show>
                </div>
            }
        } else {
            view! {
                <div class="m-2 lg:h-24 h-14">
                    <div
                        class=move || format!(
                            "lg:text-7xl text-4xl text-center text-gray-900 {}",
                            font_character()
                        )
                        on:click=move |_| { on_click(true) }
                    >{character}</div>
                </div>
            }
        }
    }
}

#[component]
pub fn Character(settings: ReadSignal<Settings>, props: Props) -> impl IntoView {
    let (is_clicked, on_click) = create_signal(false);
    props.view(settings, is_clicked, on_click)
}
