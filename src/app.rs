use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_mview::mview;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, SsrMode,
};

use crate::pages;

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
            Routes fallback=[pages::NotFound] {
                Route path={path!("")} view={pages::HomePage};
                ParentRoute path={path!("/g/:group")} view={pages::GroupLayout} ssr={SsrMode::Async} (
                    // TODO: actual 'select a member'
                    Route path={path!("")} view={pages::NoMemberSelected};
                    ParentRoute path={path!(":member")} view={pages::MemberLayout} (
                        Route path={path!("")} view={pages::PreferencesPage};
                        Route path={path!("timetable")} view={pages::TimetablePage};
                    )
                )
            }
        }
    }
}
