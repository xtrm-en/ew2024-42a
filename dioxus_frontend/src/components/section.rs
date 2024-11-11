use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionProps {
    id: String,
    children: Element,
    class: Option<String>
}

#[component]
pub fn Section(props: SectionProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: format!("w-full min-h-dvh px-10 flex flex-col justify-center margin-20 {:?}", props.class),

            {props.children}
        }
    }
}