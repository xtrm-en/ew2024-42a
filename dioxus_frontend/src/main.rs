#![allow(non_snake_case)]

mod api;
mod app;
mod assets;
mod components;
pub(crate) mod pages;
mod routes;

use dioxus_logger::tracing::{trace, Level};

fn init_log() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    trace!("Initializing {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

#[cfg(target_family = "wasm")]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    init_log();

    dioxus::launch(app::App);
}

#[cfg(any(windows, unix))]
fn main() {
    better_panic::install();

    init_log();
    
    dioxus_logger::tracing::error!("Running desktop mode is not supported yet!");
}
