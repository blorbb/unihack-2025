use std::time::Duration;

use leptos::{prelude::*, text_prop::TextProp};
use leptos_mview::mview;
stylance::import_style!(s, "boo.module.scss");

#[component]
pub fn Terminal(
    frames: Vec<String>,
    whitespace_padding: usize,
    columns: usize,
    rows: usize,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: TextProp,
) -> impl IntoView {
    // TODO: Consider zero-division error on defaulted input
    let (index, set_index) = signal(0);
    let frame_delay = 31u64;
    let num_frames = frames.len();
    // Timers don't work on the backend. AAAAA?? Can I client side this thing
    if cfg!(not(feature = "ssr")) {
        set_interval(
            move || {
                set_index.set((index.get() + 1) % num_frames);
            },
            Duration::from_millis(frame_delay),
        );
    }
    mview! {
        // HEADER
        // Bounding Box
        // Frame
        div {id} class={stylance::classes!(*class.get())} (
            h1 inner_html={index.get().to_string()};
            Frame
                frame={frames[index.get()].clone()}
                whitespace_padding={whitespace_padding}
                columns={columns}
                rows={rows};

        )
    }
}

#[component]
fn Frame(
    frame: String,
    whitespace_padding: usize,
    columns: usize,
    rows: usize,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: TextProp,
) -> impl IntoView {
    mview! {
        div
            class={stylance::classes!(*class.get())}
            {id}
            style=[format!("--columns: {}; --rows: {rows}; white-space: preserve nowrap; font-family: monospace;", columns+2*whitespace_padding)]
        ({frame.lines().map(|x| x.to_string()).map(|line| view!{
            <div inner_html=move || {
        let padding = " ".repeat(whitespace_padding);
        format!("{padding}{line}{padding}")}/>
        }).collect::<Vec<_>>()})

    }
}
