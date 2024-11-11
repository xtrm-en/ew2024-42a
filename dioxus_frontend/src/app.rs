use dioxus::prelude::*;
use crate::assets::*;
use crate::components::navbar::Navbar;
use crate::routes::Route;

#[component]
pub(crate) fn AppLayout() -> Element {
    let route: Route = use_route();
    let is_home = route == Route::Home {};
    rsx! {
        div {
            class: "overflow-x-hidden bg-auto bg-center bg-fixed w-full h-full text-white",
            background_image: "url({BACKGROUND_IMAGE})",

            if !is_home {
                Navbar {
                    name: "rsty".to_string(),
                    logo_uri: APP_LOGO,

                    div {
                        class: "flex flex-row gap-4 items-center",
                        Link {
                            to: Route::Home {},
                            button {   
                                "Home"
                            }
                        }
                        Link {
                            to: Route::Home { }, //TODO: figure out how to apply a fragment, dioxus seems to strip it out?
                            button {
                                "Contact"
                            }
                        }
                        Link {
                            to: Route::Blog {},
                            button {
                                "News"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}

pub(crate) fn App() -> Element {
    rsx! {
        Router::<Route> {
            config: || RouterConfig::default().history(WebHistory::default())
        }
    }
}
