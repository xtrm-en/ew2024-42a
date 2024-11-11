use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// The name of the app
    name: String,
    /// The logo of the app, displayed next to the name
    logo_uri: String,
    /// Children elements (mostly buttons)
    children: Element
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        div {
            class: "flex bg-slate-900/40 text-white text-xl flex-row items-center justify-around w-full h-min-12 bg-red absolute top-0",
            div {
                class: "flex flex-row gap-4",
                img {
                    src: props.logo_uri,
                    alt: format!("{} logo", props.name),
                    class: "max-h-12"
                },
                h1 { "{props.name}" },
            },
            {props.children}
        }
    }
}