use leptos::*;

#[component]
pub fn Page(
    index: ReadSignal<usize>,
    set_index: WriteSignal<usize>,
    page_count: Memo<usize>,
) -> impl IntoView {
    let left = move || {
        set_index(index().checked_sub(1).unwrap_or(0));
        window().scroll_with_x_and_y(0.0, 0.0);
    };
    let right = move || {
        set_index(usize::min(
            page_count().checked_sub(1).unwrap_or(0),
            index() + 1,
        ));
        window().scroll_with_x_and_y(0.0, 0.0);
    };
    let handle = window_event_listener(ev::keydown, move |ev| {
        match ev.key().as_str() {
            "ArrowLeft" => left(),
            "ArrowRight" => right(),
            _ => (),
        };
    });
    on_cleanup(move || handle.remove());
    view! {
        <div class="flex flex-row m-1">
            <button
                class="rounded px-3 py-2 m-1
                    border-b-4 border-l-2 shadow-lg 
                    bg-gray-700 border-gray-800 text-gray-200"
                on:click=move |_| left()
            >"Prev"</button>
            <div
                class="rounded px-3 py-2 m-1
                    border-b-4 border-l-2 shadow-lg 
                    bg-gray-800 border-gray-900 text-gray-300"
            >{ move || {
                format!("Page {} of {}", usize::min(page_count(), index() + 1), page_count())
            }}</div>
            <button
                class="rounded px-3 py-2 m-1
                    border-b-4 border-l-2 shadow-lg 
                    bg-gray-700 border-gray-800 text-gray-200"
                on:click=move |_| right()
            >"Next"</button>
        </div>
    }
}
