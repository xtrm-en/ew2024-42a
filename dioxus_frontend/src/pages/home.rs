use dioxus::prelude::*;

use crate::api::models::BlogPostItem;
use crate::components::section::Section;

#[component]
fn HeroSection() -> Element {
    rsx! {
        Section {
            id: "hero",
            class: "px-[20%]",

            h1 { class: "text-xl", "rsty" }
            h4 { class: "text-l", "your pedagogical companion, but smol" }
            div {
                class: "flex flex-row",
                a {
                    href: "#news",
                    button {
                        "Keep in touch"
                    }
                }
                a {
                    href: "#contact",
                    button {
                        "Report an issue"
                    }
                }
            }
        }
    }
}

#[component]
fn BlogSection() -> Element {
    let posts = use_resource(move || crate::api::controller::get_blog_posts(3));

    rsx! {
        Section {
            id: "blog-preview",
            class: "flex-col min-h-[50%]",
            h3 { "Keep up with the latest news!" }

            div {
                class: "flex flex-col lg:flex-row",

                match &*posts.read_unchecked() {
                    Some(Ok(list)) => rsx! {
                        for post in list {
                            PostCard { item: post.clone() }
                        }
                    },
                    Some(Err(err)) => rsx! { 
                        p { "An error occurred while fetching stories: {err}" } 
                    },
                    None => rsx! { p { "Loading items" } },
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PostCardProps {
    item: BlogPostItem,
}

#[component]
fn PostCard(props: PostCardProps) -> Element {
    rsx! {
        div {
            "{props.item.title}"
        }
    }
}

#[component]
fn FormSection() -> Element {
    rsx! {
        Section {
            id: "contact",
            "Contact section"
        }
    }
}

#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        HeroSection {},
        BlogSection {},
        FormSection {}
    }
}