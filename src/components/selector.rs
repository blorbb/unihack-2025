use std::collections::BTreeSet;

use leptos::prelude::*;
use leptos_mview::mview;

use crate::{
    clone_in,
    components::{button::ButtonVariant, Button},
};

stylance::import_style!(s, "selector.module.scss");

#[component]
pub fn Selector(
    #[prop(into)] options: Signal<BTreeSet<String>>,
    #[prop(into)] selected: Signal<BTreeSet<String>>,
    set_selected: impl Fn(BTreeSet<String>) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let show_modal = RwSignal::new(false);
    let set_selected = StoredValue::new(set_selected);
    mview! {
        button class={s::selector}
            on:click={move |_| show_modal.set(true)}
        (
            [if show_modal() {
                mview! {
                    Modal {options} {selected} {set_selected};
                }.into_any()
            } else {
                options.read()
                    .iter()
                    .filter(|opt| selected.read().contains(&**opt))
                    .map(|opt| mview!(span({opt.clone()})))
                    .collect_view()
                    .into_any()
            }]
        )
    }
}

#[component]
fn Modal(
    #[prop(into)] options: Signal<BTreeSet<String>>,
    #[prop(into)] selected: Signal<BTreeSet<String>>,
    set_selected: StoredValue<impl Fn(BTreeSet<String>) + Clone + Send + Sync + 'static>,
) -> impl IntoView {
    let select_all = move || set_selected.get_value()(options());
    let select_none = move || set_selected.get_value()(BTreeSet::new());

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
                [options()
                    .into_iter()
                    .map(|opt| mview! {
                        label(
                            input type="checkbox"
                                prop:checked={clone_in!(opt, move || selected.read().contains(&*opt))}
                                on:input={clone_in!(opt, move |ev| {
                                    let checked = event_target_checked(&ev);
                                    let mut existing = selected();
                                    if checked { existing.insert(opt.clone()); } else { existing.remove(&*opt); }
                                    set_selected.get_value()(existing)
                                })};
                            span({opt.clone()})
                        )
                    })
                    .collect_view()]
            )
        )
    }
}
