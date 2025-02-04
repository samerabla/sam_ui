use super::Action;
use dioxus::{logger::tracing::info, prelude::*};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct SubMenuState {
    pub width: Signal<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubMenu {
    id: String,
    label: &'static str,
    action: Option<Action>,
    sub_menu_list: Option<Vec<SubMenu>>,
}

impl SubMenu {
    pub fn new(label: &'static str) -> SubMenu {
        let id = sam_util::gen_id!(5, "sub_menu_");
        SubMenu {
            id,
            label,
            sub_menu_list: None,
            action: None,
        }
    }

    pub fn action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.action = Some(Action::new(f));
        self
    }

    pub fn children(mut self, sub_menu_list: Vec<SubMenu>) -> Self {
        self.sub_menu_list = Some(sub_menu_list);
        self
    }

    pub fn render(self) -> Element {
        rsx! {
            SubMenuView {
                label: self.label,
                id: self.id,
                action: self.action,
                sub_menu_list: self.sub_menu_list,
            }
        }
    }
}

#[component]
pub fn SubMenuView(
    label: &'static str,
    id: String,
    sub_menu_list: Option<Vec<SubMenu>>,
    action: Option<Action>,
) -> Element {
    let mut state = use_context_provider(|| SubMenuState {
        width: Signal::new(0.0),
    });
    let mut show = use_context::<super::MenuState>().show;
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
fn SubSubMenuWrapper(sub_menu_list: Option<Vec<SubMenu>>) -> Element {
    let show = use_context::<super::MenuState>().show;
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
