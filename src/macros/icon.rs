/////////////////////////////////////////////////
// Developer Notes
/////////////////////////////////////////////////

// To use a lib you should first add a feature in cargo.toml:
//      dioxus-free-icons = { version = "0.9", features = ["bootstrap","font-awesome-regular"] }
// All the features of dioxus-free-icons: https://crates.io/crates/dioxus-free-icons
// When no icons lib specified Lucide icons will be used

/////////////////////////////////////////////////
////// Ld Icon
/////////////////////////////////////////////////

/// A macro generates an icon using dioxus-free-icon crate.
///
/// ### Search Icon :
///
/// -  [dioxus_free_icons](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/index.html) first the name of icon you want because not all icons availabe
///
/// - [Lucide Icons](https://lucide.dev/icons/)
///
/// - [React Icons](https://react-icons.github.io/react-icons/)
///
/// - [Bootstrap icons](https://icons.getbootstrap.com/)
///
/// - [Fontawesome icons](https://fontawesome.com/search)  
///
///
/// # Examples
/// By default Lucide icons will be used
///
/// ```
///     // The order of params: [name size fill stroke]
///
///     // [Defaults:]
///     // size: 30
///     // fill: "black"
///     // stroke: "white"
///
///     // provie onyly name
///     icon!(LdX)
///
///     // provie name, size
///     icon!(LdX,60)
///
///     // provie name, size, fill
///     icon!(LdX,40,"red")
///
///     // provie name, size, fill, stroke
///     icon!(LdX,60,"white","black")
///
/// ```
///
///
/// `icon!(LdX) Expands to :`
///
/// ```
/// rsx! {
///     Icon {
///     width: 30,
///     height: 30,
///     fill: "black",
///     icon: LdX,
///     style: "stroke:'white';"
///     }
/// }
///
/// ```
#[cfg(feature = "ld")]
#[macro_export]
macro_rules! icon {
    (
        $name:ident
    ) => {{
        use dioxus_free_icons::icons::ld_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: $name,
                style: "stroke:'white';"
            }
        }
    }};

    (
        $name:ident,
        $size:expr
    ) => {{
        use dioxus_free_icons::icons::ld_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: $name,
                style: "stroke:'white';"
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr
    ) => {{
        use dioxus_free_icons::icons::ld_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                fill: $color,
                icon: $name,
                style: "stroke:'white';"
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr,
        $stroke:expr
    ) => {{
        use dioxus_free_icons::icons::ld_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                fill: $color,
                icon: $name,
                style: format!("stroke:{};",$stroke)
            }
        }
    }};
}

/////////////////////////////////////////////////
////// Io Icon
/////////////////////////////////////////////////
/////////////////////////////////////////////////
#[cfg(feature = "io")]
#[macro_export]
macro_rules! io_icon {
    (
        $name:ident
    ) => {{
        use dioxus_free_icons::icons::io_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: $name,
                style: "stroke:'white';"
            }
        }
    }};

    (
        $name:ident,
        $size:expr
    ) => {{
        use dioxus_free_icons::icons::io_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: $name,
                style: "stroke:'white';"
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr
    ) => {{
        use dioxus_free_icons::icons::io_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                fill: $color,
                icon: $name,
                style: "stroke:'white';"
            }
        }
    }};

    (
        $name:ident,
        $size:expr,
        $color:expr,
        $stroke:expr
    ) => {{
        use dioxus_free_icons::icons::io_icons::$name;
        use dioxus_free_icons::Icon;
        rsx! {
            Icon {
                width: $size,
                height: $size,
                fill: $color,
                icon: $name,
                style: format!("stroke:{};",$stroke)
            }
        }
    }};
}
