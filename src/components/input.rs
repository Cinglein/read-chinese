use crate::{
    components::{character::Props, settings::Settings},
    logic::{epub::parse_epub, segment::segment},
};
use leptos::*;
use wasm_bindgen_futures::JsFuture;

#[component]
pub fn Input(
    set_index: WriteSignal<usize>,
    set_pages: WriteSignal<Vec<Vec<Vec<Props>>>>,
    settings: ReadSignal<Settings>,
) -> impl IntoView {
    let (text, set_text) = create_signal("".to_string());
    let file_input = create_node_ref();
    view! {
        <div class="flex flex-row lg:w-[80rem] w-screen m-0 p-1">
            <label
                class=move ||
                    "flex items-center justify-center m-1 lg:w-80 w-20 lg:h-16 h-12 \
                    cursor-pointer lg:text-xl text-xs text-center rounded hover:bg-sky-50 \
                    border-slate-300 border-b-4 border-l-2 shadow-lg bg-slate-200"
            >
                <input
                    class="hidden"
                    type="file"
                    accept=".txt, .epub"
                    node_ref=file_input
                    on:input=move |_| {
                        if let Some(file) = file_input
                            .get()
                            .and_then(|f: HtmlElement<leptos::html::Input>| f.files())
                            .and_then(|f| f.get(0))
                        {
                            let words_page = settings().words_page;
                            spawn_local(async move {
                                let pages = match &file.name().split('.').last() {
                                    Some("txt") => JsFuture::from(file.text())
                                        .await
                                        .ok()
                                        .and_then(|v| v.as_string().map(|text| segment(&text, words_page))),
                                    Some("epub") => JsFuture::from(file.array_buffer())
                                        .await
                                        .ok()
                                        .and_then(|v| {
                                            let buffer = js_sys::Uint8Array::new(&v).to_vec();
                                            parse_epub(&buffer, words_page)
                                        }),
                                    _ => None
                                };
                                if let Some(pages) = pages {
                                    set_index(0);
                                    set_pages(pages);
                                } else if let Err(e) = window().alert_with_message("Failed to load file") {
                                    logging::log!("{:?}", e);
                                }
                            });

                        }
                    }
                />
                Upload .txt, .epub files
            </label>
            <textarea
                class="m-1 p-2 lg:w-[60rem] w-[10rem] lg:h-16 h-12 lg:text-xl text-xs \
                    rounded border-slate-100 border-b-4 border-l-2 shadow-lg bg-slate-100"
                placeholder="Write or paste text here"
                type="text"
                on:input=move |event| {
                    set_text(event_target_value(&event));
                }
            />
            <button
                class=move ||
                    "flex items-center justify-center m-1 lg:w-40 w-20 lg:h-16 h-12 \
                    cursor-pointer lg:text-xl text-md rounded hover:bg-sky-50 \
                    border-slate-300 border-b-4 border-l-2 shadow-lg bg-slate-200"
                on:click=move |_| {
                    let pages = text.with(|t| segment(t, settings().words_page));
                    set_index(0);
                    set_pages(pages);
                }
            >
                "Show"
            </button>
        </div>
    }
}
