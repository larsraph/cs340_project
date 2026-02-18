use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::tables;

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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use tables::TableType as TT;

    let (tt, set_tt) = signal(TT::Person);

    let cycle = move |_| match tt() {
        TT::Person => set_tt(TT::Clubs),
        TT::Clubs => set_tt(TT::Roles),
        TT::Roles => set_tt(TT::Membership),
        TT::Membership => set_tt(TT::Events),
        TT::Events => set_tt(TT::PhysicalEvents),
        TT::PhysicalEvents => set_tt(TT::VirtualEvents),
        TT::VirtualEvents => set_tt(TT::Address),
        TT::Address => set_tt(TT::Person),
    };
    let table = Resource::new(tt, |tt| async move { tables::fetch(tt).await });

    let render_table = move || table.get().map(|table| table.map(|table| table.view()));

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=cycle>"Click Me: " {move || format!("{:?}", tt.get())}</button>
        <Suspense fallback="Loading...".into_view()>{render_table}</Suspense>
    }
}
