use crate::icon;

use super::{Action, BurgerMenuState};
use dioxus::{logger::tracing::info, prelude::*};

// TODO:
// add menu separator

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Menu {
    id: String,
    label: &'static str,
    action: Option<Action>,
    sub_menu_list: Option<Vec<Menu>>,
    is_root: bool,
}

impl Menu {
    pub fn new(label: &'static str) -> Self {
        let id = sam_util::gen_id!(5, "menu_");
        Self {
            id,
            label,
            ..Menu::default()
        }
    }

    pub fn action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.action = Some(Action::new(f));
        self
    }

    pub fn children(mut self, sub_menu_list: Vec<Menu>) -> Self {
        self.sub_menu_list = Some(sub_menu_list);
        self
    }

    pub fn to_root(mut self) -> Self {
        self.is_root = true;
        self
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }

    pub fn render(self) -> Element {
        rsx! {
            MenuView {
                label: self.label,
                id: self.id,
                action: self.action,
                sub_menu_list: self.sub_menu_list,
                is_root: self.is_root,
            }
        }
    }

    pub fn render_mob(self) -> Element {
        rsx! {
            BurgerMenuView {
                label: self.label,
                id: self.id,
                action: self.action,
                sub_menu_list: self.sub_menu_list,
                is_root: self.is_root,
            }
        }
    }
}

#[derive(Clone, Copy)]
struct MenuState {
    pub show: Signal<bool>,
}

#[component]
fn MenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<Menu>>,
    action: Option<Action>,
    is_root: bool,
) -> Element {
    rsx! {
        if is_root {
            RootMenuView {
                label,
                id,
                action,
                sub_menu_list,
            }
        } else {
            SubMenuView {
                label,
                id,
                action,
                sub_menu_list,
            }
        }
    }
}

#[component]
fn RootMenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<Menu>>,
    action: Option<Action>,
) -> Element {
    let mut state = use_context_provider(|| MenuState {
        show: Signal::new(false),
    });
    let mut opened_menu: Signal<String> = use_context::<super::MenuBarState>().opened_menu;

    let id_clone_1 = id.clone();
    let id_clone_2 = id.clone();

    let handler = move |_: Event<MouseData>| {
        state.show.toggle();
        if (state.show)() {
            opened_menu.set(id_clone_1.clone())
        }
    };

    let click_handler = {
        let mut handler = handler.clone();
        move |e: Event<MouseData>| {
            handler(e);
            if let Some(action) = &action {
                action.call();
            }
            state.show.set(false);
        }
    };

    use_effect(move || {
        opened_menu.with(|m| {
            if m != &id_clone_2 {
                state.show.set(false);
            }
        });
    });

    rsx! {
        div { class: "menu_wrapper",

            div {
                class: "menu center",
                id: "{id}",
                z_index: 10,
                onclick: click_handler,
                onmouseenter: handler,
                "{label}"
            }
            SubMenuWrapper { show: state.show, sub_menu_list }
        }
    }
}

#[component]
fn SubMenuWrapper(show: Signal<bool>, sub_menu_list: Option<Vec<Menu>>) -> Element {
    // let mut sub_menu_wrapper: Signal<Option<web_sys::Element>> = use_signal(|| None);
    // let mut width: Signal<f64> = use_signal(|| 0.0);
    rsx! {
        if show() && sub_menu_list.is_some() {
            div { z_index: 11, class: "sub_menu_wrapper",
                for sub_menu in sub_menu_list.unwrap() {
                    {sub_menu.render()}
                }
            }
            div {
                class: "dropback",
                z_index: 9,
                onclick: move |_| show.set(false),
            }
        }
    }
}

#[derive(Clone, Copy)]
struct SubMenuState {
    pub width: Signal<f64>,
}

#[component]
fn SubMenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<Menu>>,
    action: Option<Action>,
) -> Element {
    let mut state = use_context_provider(|| SubMenuState {
        width: Signal::new(0.0),
    });
    let mut show = use_context::<MenuState>().show;
    let mut show_sub_menu = use_signal(|| false);

    let mut sub_menu: Signal<Option<web_sys::Element>> = use_signal(|| None);

    rsx! {
        div {
            class: "sub_menu",
            onmounted: move |elem: Event<MountedData>| async move {
                use dioxus::web::WebEventExt;
                sub_menu.set(Some(elem.as_web_event()));
                state.width.set(sub_menu().unwrap().get_bounding_client_rect().width());
            },
            onclick: move |_| {
                if let Some(action) = &action {
                    action.call()
                }
                show.set(false);
            },
            onmouseenter: move |_| {
                show_sub_menu.set(true);
            },
            onmouseleave: move |_| {
                show_sub_menu.set(false);
            },
            "{label}"
            if sub_menu_list.is_some() && show_sub_menu() {
                SubSubMenuWrapper { sub_menu_list }
            }
        }
    }
}

#[component]
fn SubSubMenuWrapper(sub_menu_list: Option<Vec<Menu>>) -> Element {
    let show = use_context::<MenuState>().show;
    let width = use_context::<SubMenuState>().width;
    rsx! {
        if show() {
            div {
                z_index: 11,
                class: "sub_sub_menu_wrapper",
                left: "{width()}px",
                for sub_menu in sub_menu_list.unwrap() {
                    {sub_menu.render()}
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////
////// Burger Menu
//////////////////////////////////////////////////////////

#[component]
fn BurgerMenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<Menu>>,
    action: Option<Action>,
    is_root: bool,
) -> Element {
    rsx! {
        if is_root {
            BurgerRootMenuView {
                label,
                id,
                action,
                sub_menu_list,
                is_root,
            }
        } else {
            BurgerSubMenuView {
                label,
                id,
                action,
                sub_menu_list,
            }
        }
    }
}

#[component]
fn BurgerRootMenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<Menu>>,
    action: Option<Action>,
    is_root: bool,
) -> Element {
    let mut state = use_context_provider(|| MenuState {
        show: Signal::new(false),
    });
    let mut show = use_context::<BurgerMenuState>().show;
    let has_children = use_signal(|| sub_menu_list.is_some());
    let click_handler = move |_: Event<MouseData>| {
        if has_children() {
            state.show.set(true);
        } else {
            if let Some(action) = &action {
                action.call();
                show.set(false)
            }
        }
    };
    rsx! {
        div { class: "burger_root_menu",

            div {
                class: "center",
                id: "{id}",
                z_index: 10,
                onclick: click_handler,
                "{label}"
            }
            BurgerSubMenuWrapper { show: state.show, sub_menu_list }
        }
    }
}

#[component]
fn BurgerSubMenuWrapper(show: Signal<bool>, sub_menu_list: Option<Vec<Menu>>) -> Element {
    rsx! {
        if show() && sub_menu_list.is_some() {
            div { z_index: 10, class: "fixed center_y_top",
                div { class: "icon_wrapper",
                    div { class: "icon", onclick: move |_| show.set(false),
                        {icon!(LdCornerDownLeft, 40, "white", "black")}
                    }
                }
                div {
                    for sub_menu in sub_menu_list.unwrap() {
                        {sub_menu.render_mob()}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct BurgerSubMenuState {
    pub show: Signal<bool>,
}

#[component]
fn BurgerSubMenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<Menu>>,
    action: Option<Action>,
) -> Element {
    let mut state = use_context_provider(|| BurgerSubMenuState {
        show: Signal::new(false),
    });
    // let mut show_sub_menu = use_signal(|| false);
    let mut show = use_context::<BurgerMenuState>().show;
    let has_children = use_signal(|| sub_menu_list.is_some());
    let click_handler = move |_: Event<MouseData>| {
        if has_children() {
            state.show.set(true);
        } else {
            if let Some(action) = &action {
                action.call();
                show.set(false)
            }
        }
    };

    rsx! {
        div {
            id: "BurgerSubMenuView",
            class: "center",
            onclick: click_handler,
            "{label}"
        }
        if sub_menu_list.is_some() && (state.show)() {
            BurgerSubSubMenuWrapper { show: state.show, sub_menu_list }
        }
    }
}

#[component]
fn BurgerSubSubMenuWrapper(show: Signal<bool>, sub_menu_list: Option<Vec<Menu>>) -> Element {
    //let mut show = use_context::<MenuState>().show;
    //let mut show_sub_menu = use_context::<BurgerSubMenuState>().show;
    rsx! {
        // if show() && show_sub_menu() {
        div {
            id: "BurgerSubSubMenuWrapper",
            z_index: 11,
            class: "fixed hoooon",
            div { class: "icon_wrapper",
                div {
                    class: "icon",
                    onclick: move |_| {
                        show.set(false);
                    },
                    {icon!(LdCornerDownLeft, 40, "white", "purple")}
                }
            }
            div {
                for sub_menu in sub_menu_list.unwrap() {
                    {sub_menu.render_mob()}
                }
            }
        }
    }
}

// #[component]
// fn SubBurgerMenuWrapper(
//     label: &'static str,
//     id: String,
//     sub_menu_list: Option<Vec<Menu>>,
//     action: Option<Action>,
//     is_root: bool,
// ) -> Element {
//     rsx! {
//         div { class: "burger_menu_wrapper",

//             for sub_menu in sub_menu_list {

//             }
//         }
//     }
// }
// use_effect(move || {
//     let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
//         if let Some(el) = div() {
//             if let Some(target) = event.target() {
//                 if let Some(target) = target.dyn_ref::<web_sys::Element>() {
//                     if !el.contains(Some(target)) {
//                         info!("ouuuut");
//                         //show.set(false);
//                     } else {

//                         info!("innnnn");
//                     }

//                 }
//             }
//         }
//     }) as Box<dyn FnMut(_)>);

//     window()
//         .expect("window should exist")
//         .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
//         .expect("should add click listener");

//     closure.forget();
// });

// if show() && sub_menu_list.is_some() {
//     div {
//         id: "samoooo",
//         class: "sub_menu_wrapper",
//         onmounted: move |elem: Event<MountedData>| {
//             use dioxus::web::WebEventExt;
//             //info!("{:#?}",elem.data().as_web_event().id());
//             div.set(Some(elem.data().as_web_event()));
//         },
//         for sub_menu in sub_menu_list.unwrap() {
//                 div {
//                     {
//                         sub_menu.render()
//                     }
//                 }

//         }
//     }
// }

// onmounted: move |elem: Event<MountedData>| async move {
//     use dioxus::web::WebEventExt;
//     menu_wrapper.set(Some(elem.data().as_web_event()));
//     info!("bottom {:#?}",menu_wrapper().unwrap().get_bounding_client_rect().bottom());
//     //info!("height {:#?}",menu_wrapper().unwrap().client_height());
// },

// onmounted: move |elem: Event<MountedData>| async move {
//     use dioxus::web::WebEventExt;
//     sub_menu_wrapper.set(Some(elem.as_web_event()));
//     width.set(sub_menu_wrapper().unwrap().get_bounding_client_rect().width());
//     info!(
//         "width {:#?}", sub_menu_wrapper().unwrap().get_bounding_client_rect().width()
//     );
// },
