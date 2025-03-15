use std::collections::HashSet;

use leptos::prelude::*;
use leptos_mview::mview;

use crate::components::{button::ButtonVariant, Button};

stylance::import_style!(s, "selector.module.scss");

#[component]
pub fn Selector(
    #[prop(into)] options: Signal<Vec<String>>,
    #[prop(into)] selected: Signal<HashSet<usize>>,
    #[prop(into)] set_selected: impl Fn(HashSet<usize>) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let show_modal = RwSignal::new(false);
    let set_selected = StoredValue::new(set_selected);
    mview! {
        button class={s::selector}
            on:click={move |_| show_modal.set(!show_modal())}
        (
            [if show_modal() {
                mview! {
                    Modal {options} {selected} {set_selected};
                }.into_any()
            } else {
                options.read()
                    .iter().enumerate()
                    .filter(|(i, _)| selected.read().contains(i))
                    .map(|(_, opt)| mview!(span({opt.clone()})))
                    .collect_view()
                    .into_any()
            }]
        )
    }
}

#[component]
fn Modal(
    #[prop(into)] options: Signal<Vec<String>>,
    #[prop(into)] selected: Signal<HashSet<usize>>,
    #[prop(into)] set_selected: StoredValue<
        impl Fn(HashSet<usize>) + Clone + Send + Sync + 'static,
    >,
) -> impl IntoView {
    let select_all = move || set_selected.get_value()((0..options.read().len()).collect());
    let select_none = move || set_selected.get_value()(HashSet::new());

    mview! {
        div class={s::modal}
        (
            div class={s::modal_heading} (
                Button variant={ButtonVariant::Secondary}
                    on:click={move |_| select_all()}
                ("Select all")
                Button variant={ButtonVariant::Secondary}
                    on:click={move |_| select_none()}
                ("Deselect all")
            )
            div class={s::modal_checkboxes} (
                {options.read()
                    .iter().enumerate()
                    .map(|(i, opt)| mview! {
                        label(
                            input type="checkbox"
                                prop:checked=[selected.read().contains(&i)]
                                on:input={move |ev| {
                                    let checked = event_target_checked(&ev);
                                    let mut existing = selected();
                                    if checked { existing.insert(i); } else { existing.remove(&i); }
                                    set_selected.get_value()(existing)
                                }};
                            span({opt.clone()})
                        )
                    })
                    .collect_view()}
            )
        )
    }
}
