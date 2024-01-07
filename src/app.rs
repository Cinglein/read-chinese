use crate::{
    components::{input::Input, page::Page, reader::Reader, settings::Settings, top::Top},
    error_template::{AppError, ErrorTemplate},
    logic::storage::get_settings,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/read-chinese.css"/>

        // sets the document title
        <Title text="Read Chinese Online"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link
            href="https://fonts.googleapis.com/css2?\
                family=Noto+Sans+SC:wght@300&\
                family=Noto+Serif+SC&\
                family=Liu+Jian+Mao+Cao&\
                display=swap"
            rel="stylesheet"
        />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (settings, set_settings) = create_signal(Default::default());
    let (pages, set_pages) = create_signal(Vec::new());
    let (index, set_index) = create_signal(0usize);
    let page_count = create_memo(move |_| pages.with(|p| p.len()));
    let init = create_render_effect(move |_| {
        if let Some(settings) = get_settings() {
            set_settings(settings);
        }
    });
    init.dispose();
    view! {
        <main>
            <div class="flex flex-col min-h-screen">
                <Top/>
                <Input set_index=set_index set_pages=set_pages settings=settings/>
                <Settings settings=settings set_settings=set_settings/>
                <Reader index=index pages=pages settings=settings/>
                <div class="sticky z-100 bottom-0 mx-auto">
                    <Page index=index set_index=set_index page_count=page_count/>
                </div>
            </div>
        </main>
    }
}
