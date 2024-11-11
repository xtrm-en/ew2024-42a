use crate::pages::*;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}