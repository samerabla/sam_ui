use dioxus::{logger::tracing::info, prelude::*};

use super::menu::Menu;

#[derive(Clone, Copy)]
pub(crate) struct MenuBarState {
    pub opened_menu: Signal<String>,
}

#[component]
pub fn MenuBar(menu_list: Vec<Menu>) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    use_context_provider(|| MenuBarState {
        opened_menu: Signal::new("".to_string()),
    });
    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        div { class: "menu_bar",
            for menu in menu_list {
                {menu.render()}
            }
        }
    }
}
