use crate::pages::home::Home;
use crate::pages::blog::Blog;
use crate::app::AppLayout;

use dioxus::prelude::*;

#[component]
fn BlogPost(id: usize) -> Element {
    return rsx! {
        p { "hi im post {id}" }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[layout(AppLayout)]

    #[nest("/blog")]
        #[nest("/:id")]
            #[route("/")]
            BlogPost { id: usize },
        #[end_nest]
        #[route("/")]
        Blog { },
    #[end_nest]
    #[route("/")]
    Home { },
}