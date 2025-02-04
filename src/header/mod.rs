#![allow(non_snake_case)]
// use dioxus::prelude::*;
use std::{fmt::Debug, rc::Rc};

mod menu_bar;
pub use menu_bar::*;

mod menu_list;
pub use menu_list::*;

mod menu;
pub use menu::*;

// mod sub_menu;
// pub use sub_menu::*;

pub struct Action(Rc<dyn Fn()>);

impl Action {
    fn new<T: Fn() + 'static>(f: T) -> Self {
        Self(Rc::new(f))
    }
    fn call(&self) {
        (self.0)()
    }
}

impl Clone for Action {
    fn clone(&self) -> Self {
        Action(self.0.clone())
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action")
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
