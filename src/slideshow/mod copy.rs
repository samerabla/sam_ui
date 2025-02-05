use animation::Animated;
use dioxus::{logger::tracing::info, prelude::*};
//use dioxus_sdk::utils::timing::{use_debounce, use_interval};
use animation::animate;
use animation::Animation;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

pub mod animation;

const CSS: Asset = asset!("/assets/slideshow.css");

pub struct Slideshow {
    slides: Vec<&'static str>,
    amimation: Animation,
    slide_duration: u64,
}

impl Slideshow {
    pub fn new(slides: Vec<&'static str>) -> Self {
        Self {
            slides,
            amimation: Animation::SlideXLeft,
            slide_duration: 3_000,
        }
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.amimation = animation;
        self
    }

    pub fn render(self) -> Element {
        rsx! {
            SlideshowView {
                slides: self.slides,
                animation: self.amimation,
                slide_duration: self.slide_duration,
            }
        }
    }
}

#[component]
pub fn SlideshowView(
    slides: Vec<&'static str>,
    animation: Animation,
    //anim_duration: &'static str,
    slide_duration: u64,
) -> Element {
    let last = Rc::new(slides.len() - 1);
    let animation = Rc::new(animation);
    let mut forward = use_signal(|| true);
    let mut in_anim_class = use_signal(|| "");
    let mut out_anim_class = use_signal(|| "");
    // let mut in_anim_class_back = use_signal(|| "");
    // let mut out_anim_class_back = use_signal(|| "");

    let mut current_slide: Signal<isize> = use_signal(|| -1);

    let next_slide: Memo<isize> = use_memo({
        let last = last.clone();
        move || {
            if forward() {
                if current_slide() == *last as isize {
                    0
                } else {
                    current_slide() + 1
                }
            } else {
                if current_slide() == 0 {
                    *last as isize
                } else {
                    current_slide() - 1
                }
            }
        }
    });

    let move_slide = Rc::new(RefCell::new({
        let last = last.clone();
        move || {
            current_slide.with_mut(|curr| {
                *curr = if forward() {
                    if *curr == *last as isize {
                        0
                    } else {
                        *curr + 1
                    }
                } else {
                    if *curr == 0 {
                        *last as isize
                    } else {
                        *curr - 1
                    }
                };
            });
        }
    }));

    let mut interval = use_interval(Duration::from_millis(slide_duration), {
        let move_slide = move_slide.clone();
        let animation = animation.clone();
        move || {
            move_slide.borrow_mut()();
            // in_anim_class.set("slideInLeft");
            // out_anim_class.set("slideOutRight");
            in_anim_class.set(animation.get().enter);
            out_anim_class.set(animation.get().leave);
        }
    });

    // let mut interval = spawn({
    //     let move_slide = move_slide.clone();
    //     async move {
    //         loop {
    //             gloo_timers::future::sleep(Duration::from_secs(3)).await;
    //             move_slide.borrow_mut()();
    //             in_anim_class.set("slideInLeft");
    //             out_anim_class.set("slideOutRight");
    //         }
    //     }
    // });

    let mut move_slide_manual = {
        let move_slide = move_slide.clone();
        let animation = animation.clone();

        move || {
            interval.cancel();
            if !forward() {
                forward.set(true);
                move_slide.borrow_mut()();
                move_slide.borrow_mut()();
            }
            move_slide.borrow_mut()();
            in_anim_class.set(animation.get().enter);
            out_anim_class.set(animation.get().leave);
            // spawn(async move {
            //     gloo_timers::future::sleep(Duration::from_secs(3)).await;
            //     interval.resume();
            // });
        }
    };

    let mut back = {
        let move_slide = move_slide.clone();
        let animation = animation.clone();
        move || {
            interval.cancel();
            if forward() {
                forward.set(false);
                move_slide.borrow_mut()();
                move_slide.borrow_mut()();
            }
            move_slide.borrow_mut()();
            in_anim_class.set(animation.get().enter_back);
            out_anim_class.set(animation.get().leave_back);
        }
    };

    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            class: "slideshow",
            width: "1000px",
            height: "500px",
            max_width: "100%",
            div { class: "bg absolute" }
            for (id , src) in slides.iter().enumerate() {
                if id as isize == current_slide() {
                    Slide { //----
                        src,
                        id,
                        // anim_class: in_anim_class,
                        anim_class: out_anim_class,
                        z_index: 0,
                    }
                } else if id as isize == next_slide() {
                    Slide {
                        src,
                        id,
                        // anim_class: out_anim_class,
                        anim_class: in_anim_class,
                        z_index: 1,
                    }
                }
            }
        }
        button { onclick: move |_| back(), "<<<" }
        button { onclick: move |_| move_slide_manual(), ">>>" }

        p { "{next_slide()}" }
    }
}

#[component]
pub fn Slide(src: String, id: usize, anim_class: String, z_index: usize) -> Element {
    rsx! {
        div { class: "absolute {anim_class}", z_index,
            img { src, loading: "lazy" }
        }
    }
}

//---------------------------------------
use dioxus::prelude::{use_hook, Callback, Writable};

#[derive(Clone, PartialEq, Copy)]
pub struct UseInterval {
    inner: dioxus::prelude::Signal<InnerUseInterval>,
}

struct InnerUseInterval {
    pub(crate) interval: Option<dioxus::prelude::Task>,
}

impl Drop for InnerUseInterval {
    fn drop(&mut self) {
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }
    }
}

impl UseInterval {
    /// Cancel the interval
    pub fn cancel(&mut self) {
        if let Some(interval) = self.inner.write().interval.take() {
            interval.cancel();
        }
    }

    pub fn pause(&self) {
        if let Some(interval) = self.inner.read().interval {
            interval.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(interval) = self.inner.read().interval {
            interval.resume();
        }
    }

    pub fn wake(&self) {
        if let Some(interval) = self.inner.read().interval {
            interval.wake();
        }
    }
}

/// Repeatedly calls a function every a certain period.
pub fn use_interval(period: Duration, mut action: impl FnMut() + 'static) -> UseInterval {
    let inner = use_hook(|| {
        let callback = Callback::new(move |()| {
            action();
        });

        dioxus::prelude::Signal::new(InnerUseInterval {
            interval: Some(dioxus::prelude::spawn(async move {
                loop {
                    gloo_timers::future::sleep(period).await;

                    callback.call(());
                }
            })),
        })
    });

    UseInterval { inner }
}
