use crate::components::*;
use leptos::*;

#[component]
pub fn Reader(
    index: ReadSignal<usize>,
    pages: ReadSignal<Vec<Vec<Vec<character::Props>>>>,
    settings: ReadSignal<settings::Settings>,
) -> impl IntoView {
    view! {
        <div class="mx-auto lg:w-[60rem] w-screen max-w-full grow">
            {
                move || pages.with(|p| p
                    .get(index())
                    .map(|sentences| sentences
                        .iter()
                        .map(|sentence| {
                            view! {
                                <div class="flex flex-wrap justify-start lg:my-6 my-4">
                                    {
                                    sentence
                                    .iter()
                                    .map(|props| view! {
                                        <character::Character settings=settings props={props.clone()}/>
                                    })
                                    .collect_view()
                                    }
                                </div>
                            }
                        })
                        .collect_view()
                    )
                    .unwrap_or(Default::default())
                )
            }
        </div>
    }
}
