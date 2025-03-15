#![feature(try_blocks)]
#![feature(let_chains)]

/// Put everything that is server-only in this module.
///
/// This has access to server-only crates.
#[cfg(feature = "ssr")]
mod server;
#[cfg(feature = "ssr")]
pub use server::api;
#[cfg(feature = "ssr")]
pub use server::classes;
#[cfg(feature = "ssr")]
pub use server::solver;
/// Put things that will be shared between the server
/// and frontend. This should just be a bunch of types
/// that are used on the API calls.
mod shared;
pub use shared::*;
const TESTING: bool = true;
