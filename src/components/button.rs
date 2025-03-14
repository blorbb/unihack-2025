use leptos::{prelude::*, text_prop::TextProp};
use leptos_mview::mview;

stylance::import_style!(s, "button.module.scss");

pub enum ButtonVariant {
    Primary,
    Secondary,
}

#[component]
pub fn Button(
    variant: ButtonVariant,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] r#type: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => s::primary,
        ButtonVariant::Secondary => s::secondary,
    };

    mview! {
        button
            class={stylance::classes!(s::btn, variant_class, *class.get())}
            {id}
            {type}
        ({children()})
    }
}
