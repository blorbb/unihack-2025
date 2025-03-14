use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_mview::mview;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

stylance::import_style!(style, "app.module.scss");

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

    mview! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        Stylesheet #leptos href="/pkg/unihack.css";
        Title text="Welcome";

        Router {
            main {
                Routes fallback=["Page not found".into_view()] {
                    Route path={path!("")} view={HomePage};
                }
            }
        }
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let action = ServerAction::<Increment>::new();
    let on_click = move |_| {
        action.dispatch("hello there".to_string().into());
    };

    mview! {
        h1 { "Something or another" }
        button class={style::some_button} on:click={on_click} {
            "Click me: " {action.value()}
        }
    }
}

#[server]
async fn increment(msg: String) -> Result<u32, ServerFnError> {
    Ok(backend::increment(&msg))
}
