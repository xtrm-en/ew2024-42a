use dioxus::prelude::*;

#[component]
pub(crate) fn Blog() -> Element {
    let posts = use_resource(move || crate::api::controller::get_blog_posts(10));

    match &*posts.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for news in list {
                    p { "{news.title}" }
                }
            }
        },
        Some(Err(err)) => rsx! { "An error occurred while fetching posts: {err}" },
        None => rsx! { "Loading..." },
    }
}