use super::{Action, BurgerMenuState, BurgerMenuWrapper};
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
            if (state.show)() {
                BurgerMenuWrapper { show: state.show, menu_list: sub_menu_list.unwrap() }
            }
        }
    }
}
