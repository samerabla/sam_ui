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
            if (state.show)() && sub_menu_list.is_some() {
                BurgerMenuWrapper { show: state.show, menu_list: sub_menu_list.unwrap() }
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
        if (state.show)() && sub_menu_list.is_some() {
            BurgerMenuWrapper { show: state.show, menu_list: sub_menu_list.unwrap() }
        }
    }
}
