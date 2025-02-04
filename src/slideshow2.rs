use dioxus::{logger::tracing::info, prelude::*};
//use dioxus_sdk::utils::timing::{use_debounce, use_interval};
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

const CSS: Asset = asset!("/assets/slideshow.css");

#[component]
pub fn Slideshow(
    slides: Vec<&'static str>,
    anim_duration: &'static str,
    show_duration: u32,
) -> Element {
    let last = Rc::new(slides.len() - 1);
    let last_clone = last.clone();
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

    // let mut interval = use_interval(Duration::from_secs(3), {
    //     let move_slide = move_slide.clone();
    //     move || {
    //         move_slide.borrow_mut()();
    //         in_anim_class.set("slideInLeft");
    //         out_anim_class.set("slideOutRight");
    //     }
    // });

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

    //*************************************************** */
    // Interval state
    let mut interval: Signal<Option<Task>> = use_signal(|| None);

    fn start_new_interval(
        move_slide: Rc<RefCell<impl FnMut() + 'static>>,
        mut in_anim_class: Signal<&str>,
        mut out_anim_class: Signal<&str>,
        duration: Duration,
    ) {
        let handle = dioxus::prelude::spawn(async move {
            loop {
                gloo_timers::future::sleep(duration).await;

                // Update slide and animation signals
                move_slide.borrow_mut()();
                in_anim_class.set("slideInLeft");
                out_anim_class.set("slideOutRight");
            }
        });

        // Store the interval handle in the signal
        interval.set(Some(handle));
    }

    use_effect({
        let move_slide = move_slide.clone();
        let in_anim_class = in_anim_class.clone();
        let out_anim_class = out_anim_class.clone();
        move || {
            start_new_interval(
                move_slide.clone(),
                in_anim_class.clone(),
                out_anim_class.clone(),
                Duration::from_secs(show_duration.into()),
            );

            // // Cleanup: stop the interval when the component unmounts
            // (move || {
            //     if let Some(handle) = interval.write().take() {
            //         handle.cancel();
            //     }
            // })()
        }
    });

    // Restart the interval manually when needed
    let restart_interval = {
        let move_slide = move_slide.clone();
        // let mut interval = interval.clone();
        let in_anim_class = in_anim_class.clone();
        let out_anim_class = out_anim_class.clone();
        move || {
            // Stop the current interval
            if let Some(handle) = interval.write().take() {
                handle.cancel();
            }

            // Start a new interval
            start_new_interval(
                move_slide.clone(),
                in_anim_class.clone(),
                out_anim_class.clone(),
                Duration::from_secs(show_duration.into()),
            );
        }
    };

    // Manual forward slide
    let mut move_slide_manual = {
        let move_slide = move_slide.clone();
        let mut restart_interval = restart_interval.clone();
        move || {
            restart_interval(); // Restart the interval
            forward.set(true); // Ensure forward direction
            move_slide.borrow_mut()(); // Move forward
            in_anim_class.set("slideInLeft");
            out_anim_class.set("slideOutRight");
        }
    };

    // Manual backward slide
    let mut back = {
        let move_slide = move_slide.clone();
        let mut restart_interval = restart_interval.clone();
        move || {
            restart_interval(); // Restart the interval
            forward.set(false); // Ensure backward direction
            move_slide.borrow_mut()(); // Move backward
            in_anim_class.set("slideInRight");
            out_anim_class.set("slideOutLeft");
        }
    };

    //************************************************** */
    // let mut move_slide_manual = {
    //     let move_slide = move_slide.clone();
    //     move || {
    //         interval.cancel();
    //         if !forward() {
    //             forward.set(true);
    //             move_slide.borrow_mut()();
    //             move_slide.borrow_mut()();
    //         }
    //         move_slide.borrow_mut()();
    //         in_anim_class.set("slideInLeft");
    //         out_anim_class.set("slideOutRight");
    //         interval.wake();
    //     }
    // };

    // let mut back = {
    //     let move_slide = move_slide.clone();
    //     move || {
    //         interval.cancel();
    //         if forward() {
    //             forward.set(false);
    //             move_slide.borrow_mut()();
    //             move_slide.borrow_mut()();
    //         }
    //         move_slide.borrow_mut()();
    //         in_anim_class.set("slideInRight");
    //         out_anim_class.set("slideOutLeft");
    //     }
    // };

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
                    Slide {
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
