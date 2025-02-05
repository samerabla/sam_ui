use std::{cell::RefCell, rc::Rc};

use dioxus::{logger::tracing::info, prelude::*};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, Event, HtmlCollection, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit,
};

const CSS: Asset = asset!("/assets/slideshow.css");

/////////////////////////////////////////////////////////////////////

pub struct Elem(pub Option<web_sys::Element>);

impl Elem {
    pub fn to_elem(selector: &str) -> Option<web_sys::Element> {
        Self::from(selector).0
    }

    pub fn to_html_elem(selector: &str) -> Option<web_sys::HtmlElement> {
        if let Some(el) = Self::from(selector).0 {
            el.dyn_into::<web_sys::HtmlElement>().ok()
        } else {
            None
        }
    }
    pub fn remove_class(&self, name: &str) {
        if let Some(elem) = &self.0 {
            elem.class_list().remove_1(name).ok();
        }
    }
}

/// Implement conversion from `&str` (CSS selector)
impl From<&str> for Elem {
    fn from(selector: &str) -> Self {
        let element = window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.query_selector(selector).ok().flatten());
        Elem(element)
    }
}

impl From<String> for Elem {
    fn from(selector: String) -> Self {
        Elem::from(selector.as_str()) // Convert `String` to `&str`
    }
}

impl From<&String> for Elem {
    fn from(selector: &String) -> Self {
        Elem::from(selector.as_str()) // Convert `&String` to `&str`
    }
}

/// Implement conversion from `Element`
impl From<web_sys::Element> for Elem {
    fn from(element: web_sys::Element) -> Self {
        Elem(Some(element))
    }
}

pub fn animate_staggered<T>(selector: T, from: &str, to: &str)
where
    T: Into<Elem>,
{
    animate_staggered_full(selector, from, to, "", "", 50, None::<fn()>);
}

pub fn animate_staggered_full<T>(
    selector: T,
    from: &str,
    to: &str,
    from_style: &str,
    to_style: &str,
    interval: u32,
    callback: Option<impl FnMut() + 'static>,
) where
    T: Into<Elem>,
{
    if let Some(elem) = selector.into().0 {
        let child_elements: HtmlCollection = elem.children();
        let callback = callback.map(|cb| Rc::new(RefCell::new(cb)));

        for i in 0..child_elements.length() {
            if let Some(child) = child_elements.item(i) {
                let child = Rc::new(RefCell::new(child));
                let mut style = String::from("");

                // Add the initial class if exist
                if !from.is_empty() {
                    child.borrow_mut().class_list().add_1(from).ok();
                }

                // Add the animation class if exist
                if !to.is_empty() {
                    child.borrow_mut().class_list().add_1(to).ok();
                }

                // Add the delay style
                let delay_style = format!("animation-delay: {}ms;", i * interval);
                style.push_str(delay_style.as_str());

                // Add the initial style if exist
                if !from_style.is_empty() {
                    style.push_str(from_style);
                }

                // Add the animation style if exist for modifying only dynamic properties
                if !to_style.is_empty() {
                    style.push_str(to_style);
                }

                // Handle the origin style in order to not be deleted
                if let Some(original_style) = child.borrow_mut().get_attribute("style") {
                    let stl: Vec<&str> = original_style.split(&style).collect();
                    let origin = stl.get(0);
                    if let Some(origin) = origin {
                        if !origin.is_empty() {
                            let new_style = format!("{origin}{style}");
                            style.clear();
                            style.push_str(new_style.as_str());
                        }
                    }
                }

                // Add the aggregated style
                child
                    .borrow_mut()
                    .set_attribute("style", style.as_str())
                    .ok();

                let child_clone = child.clone();
                let to_owned = to.to_string(); // Convert &str to String
                let from_owned = from.to_string();
                // if let Some(func) = callback.clone() {
                // }
                // Create an event listener for 'animationend'
                let closure = Closure::wrap(Box::new({
                    let child_clone = child_clone.clone();
                    let to = to_owned.clone(); // Move owned String inside closure
                    let from = from_owned.clone();
                    let cb = callback.clone();
                    move |_: Event| {
                        //----------------------------
                        let child = child_clone.borrow_mut();
                        // Remove the added classes
                        child.class_list().remove_1(&to).ok();
                        child.class_list().remove_1(&from).ok();

                        // Remove the "class" attribute if it's empty (or only whitespace)
                        if let Some(class_value) = child.get_attribute("class") {
                            if class_value.trim().is_empty() {
                                child.remove_attribute("class").ok();
                            }
                        }

                        // Remove the added style
                        if let Some(style) = child.get_attribute("style") {
                            let stl: Vec<&str> = style.split("animation-delay").collect();
                            if let Some(origin) = stl.get(0) {
                                if !origin.is_empty() {
                                    child.set_attribute("style", origin).ok();
                                }
                            }
                        }

                        if let Some(func) = cb.clone() {
                            func.borrow_mut()();
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                // Attach event listener to the element
                child
                    .borrow_mut()
                    .add_event_listener_with_callback(
                        "animationend",
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                // Keep closure alive until event triggers
                closure.forget();
            }
        }
    }
}

pub fn animate<T>(
    selector: T,
    animation_class: &str,
    animation_style: Option<&str>,
    callback: Option<impl FnMut() + 'static>,
) where
    T: Into<Elem>,
{
    // Apply the animation class
    if let Some(elem) = selector.into().0 {
        elem.class_list().add_1(animation_class).ok();
        //.expect("Failed to add animation class");

        // Modify only dynamic properties
        if let Some(style) = animation_style {
            elem.set_attribute("style", style)
                .expect("Failed to set style attributes");
        }

        if let Some(mut func) = callback {
            // Create an event listener for 'animationend'
            let closure = Closure::wrap(Box::new(move |_: Event| {
                func();
            }) as Box<dyn FnMut(_)>);

            // Attach event listener to the element
            elem.add_event_listener_with_callback("animationend", closure.as_ref().unchecked_ref())
                .expect("Failed to add event listener");

            // Keep closure alive until event triggers
            closure.forget();
        }
    }
}

///////////////////////////////////////////////////////
// Animated
///////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum RunAnimation {
    OnMounted,
    OnHover,
}

#[derive(PartialEq, Props, Clone)]
pub struct AnimatedProps {
    #[props(default = "")]
    from: &'static str,

    #[props(default = "")]
    to: &'static str,

    #[props(default = "")]
    from_style: &'static str,

    #[props(default = "")]
    to_style: &'static str,

    #[props(default = 50)]
    interval: u32,

    children: Element,

    #[props(default = RunAnimation::OnMounted)]
    run: RunAnimation,
}

pub fn AnimatedOnHover(props: AnimatedProps) -> Element {
    rsx! {
        Animated { to: props.to, from: props.from, run: RunAnimation::OnHover, {props.children} }
    }
}

#[component]
pub fn Animated(props: AnimatedProps) -> Element {
    let id = use_memo(|| sam_util::gen_id!());
    let id_selector = use_memo(move || format!("#{}", id()));
    // let mut elem = use_signal(|| None);
    //let initial = props.from.unwrap_or("");
    let animate = Callback::new(move |()| {
        animate(
            id_selector(),
            props.to,
            None,
            Some(move || {
                Elem::from(id_selector()).remove_class(props.to);
            }),
        )
    });
    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            id: id(),
            class: "{props.from}",
            onmounted: {
                let run = props.run.clone();
                move |_| {
                    if run == RunAnimation::OnMounted {
                        animate.call(())
                    }
                }
            },
            onmouseenter: {
                let run = props.run.clone();
                move |_| {
                    if run == RunAnimation::OnHover {
                        animate.call(());
                    }
                }
            },
            {props.children}
        }
    }
}

#[component]
pub fn AnimatedStaggered(props: AnimatedProps) -> Element {
    let id = use_memo(|| sam_util::gen_id!());
    let id_selector = use_memo(move || format!("#{}", id()));
    let animate = Callback::new(move |()| {
        animate_staggered_full(
            id_selector(),
            props.from,
            props.to,
            props.from_style,
            props.to_style,
            props.interval,
            None::<fn()>,
        );
    });
    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            id: id(),
            //class: "{props.from}",
            onmounted: {
                let run = props.run.clone();
                move |_| {
                    if run == RunAnimation::OnMounted {
                        animate.call(())
                    }
                }
            },
            onmouseenter: {
                let run = props.run.clone();
                move |_| {
                    if run == RunAnimation::OnHover {
                        animate.call(());
                    }
                }
            },
            {props.children}
        }
    }
}

#[component]
pub fn AnimatedOnDisplay(props: AnimatedProps) -> Element {
    let mut elem: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let mut is_visible = use_signal(|| false);
    let mut observer: Signal<Option<IntersectionObserver>> = use_signal(|| None);

    use_effect(move || {
        let observer_callback = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _: JsValue| {
            let entry: IntersectionObserverEntry = entries[0].clone().into();
            if entry.is_intersecting() {
                is_visible.set(true);
            } else {
                is_visible.set(false);
            }
        }) as Box<dyn FnMut(Vec<JsValue>, JsValue)>);

        let observer_options = IntersectionObserverInit::new();
        // observer_options.set_threshold(&JsValue::from_f64(0.5));

        let _observer = IntersectionObserver::new_with_options(
            observer_callback.as_ref().unchecked_ref(),
            &observer_options,
        )
        .expect("Failed to create observer");

        // let _observer = IntersectionObserver::new_with_options(
        //     observer_callback.as_ref().unchecked_ref(),
        //     &IntersectionObserverInit::new(),
        // )
        // .expect("Failed to create observer");

        observer.set(Some(_observer));

        //observer.observe(&element);
        if let Some(observer) = observer() {
            if let Some(elem) = elem() {
                observer.observe(&elem);
            }
        }

        observer_callback.forget();
    });

    if is_visible() {
        if let Some(elem) = elem() {
            animate(elem, props.to, None, None::<fn()>)
        }
    } else {
        if let Some(elem) = elem() {
            elem.class_list().remove_1(props.to).ok();
        }
    }
    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            class: "{props.from}",
            onmounted: move |evt| {
                use dioxus::web::WebEventExt;
                elem.set(Some(evt.as_web_event()));
            },
            {props.children}
        }
    }
}
