use dioxus::{logger::tracing::info, prelude::*};

use super::menu::Menu;
use crate::icon;

#[derive(Clone, Copy)]
pub struct BurgerMenuState {
    pub show: Signal<bool>,
}

#[component]
pub fn MenuList(menu_list: Vec<Menu>) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let mut state = use_context_provider(|| BurgerMenuState {
        show: Signal::new(false),
    });
    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        div { id: "burgerMenu", onclick: move |_| state.show.set(true), {icon!(LdMenu)} }
        if (state.show)() && !menu_list.is_empty() {
            BurgerMenuWrapper { show: state.show, menu_list }
        }
    }
}

#[component]
pub fn BurgerMenuWrapper(show: Signal<bool>, menu_list: Vec<Menu>) -> Element {
    rsx! {
        div {
            z_index: 10,
            class: "fixed center_y_top",
            id: "BurgerMenuWrapper",
            //// pick the first menu and check if root to define the icon: x or arrow
            if let Some(menu) = menu_list.get(0) {
                div { class: "icon_wrapper",
                    div { class: "icon", onclick: move |_| show.set(false),
                        {
                            if menu.is_root() {
                                icon!(LdX, 40)
                            } else {
                                icon!(LdCornerDownLeft, 40, "white", "purple")
                            }
                        }
                    }
                }
            }
            div {
                for menu in menu_list {
                    {menu.render_mob()}
                }
            }
        }
    }
}

// #[component]
// fn BurgerMenuWrapper(show: Signal<bool>, menu_list: Option<Vec<Menu>>) -> Element {
//     rsx! {
//         if show() && menu_list.is_some() {
//             div {
//                 z_index: 10,
//                 class: "fixed center_y_top",
//                 id: "BurgerMenuWrapper",
//                 div { class: "icon_wrapper",
//                     div { class: "icon", onclick: move |_| show.set(false), {icon!(LdX, 40)} }
//                 }
//                 div {
//                     for menu in menu_list.unwrap() {
//                         {menu.render_mob()}
//                     }
//                 }
//             }
//         }
//     }
// }
