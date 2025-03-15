use leptos::{prelude::*, text_prop::TextProp};
use leptos_mview::mview;
stylance::import_style!(s, "boo.module.scss");
#[component]
pub fn Terminal(
    frame: Vec<&'static str>,
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
        ({frame.into_iter().map(|line| view!{
            <div inner_html=move || {
        let padding = " ".repeat(whitespace_padding);
        format!("{padding}{line}{padding}")}/>
        }).collect::<Vec<_>>()})

    }
}
