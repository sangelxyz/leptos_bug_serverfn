use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Filter {
    pub page: Option<usize>,
    pub countries: Vec<i32>,
}

#[server(default)]
async fn load_server_fn(filter: Filter) -> Result<String, ServerFnError> {
    leptos::logging::log!("Filter: {filter:?}");

    if let Some(page) = filter.page {
        Ok(format!("hello world {}", page))
    } else {
        Ok("Page 1".to_string())
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/cargo-bon-bug.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use leptos_router::hooks::use_query_map;

    let q = use_query_map();
    let page = move || {
        q.read()
            .get("page")
            .and_then(|page| page.parse::<usize>().ok())
            .unwrap_or(1)
    };

    let async_fn = Resource::new(
        move || page(),
        |page_num| async move {
            let filter = Filter {
                page: Some(1),
                countries: vec![],
            };

            load_server_fn(filter).await
        },
    );

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Suspense fallback=move || view! { <div>"Loading"</div> }.into_any()>
        {move || match async_fn.get() {
            Some(data) => {
                //leptos::logging::log!("{data:?}");
                view!{<div>{data.unwrap()}</div>}.into_any()},
            None => view!{<div>error</div>}.into_any()
        }}
        </Suspense>
        <a href={move || format!("/?page={}", page()+1)}>"Next Page: " {page}</a>
    }
}
