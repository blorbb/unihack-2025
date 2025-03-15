#![feature(try_blocks)]
#![feature(let_chains)]

mod api;
pub mod app;
pub mod components;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[macro_export]
macro_rules! clone_in {
    ($ident:ident, $($tt:tt)*) => {
        {
            let $ident = ::std::borrow::ToOwned::to_owned(&$ident);
            $crate::clone_in!($($tt)*)
        }
    };
    ($($tt:tt)*) => {
        {$($tt)*}
    }
}
