//! All macros, internal and exported

// For our println etc. macros.
#[doc(hidden)]
pub use web_sys as _web_sys;
#[doc(hidden)]
pub use js_sys as _js_sys;


/// Allows a message in `expect` that is computed lazily, for performance on the happy path.
macro_rules! expect {
    ($inner:expr, $($args:expr,)+) => {
        expect!($inner, $($args),+)
    };
    ($inner:expr, $($args:expr),+) => {
        match $inner {
            Ok(val) => val,
            Err(e) => panic!(format!("got the error \"{:?}\" while {} - this is a bug, \
                                     please report it at https://github.com/derekdreery/dommer",
                                     e, format_args!($($args),+))),
        }
    };
    ($inner:expr) => {
        match $inner {
            Ok(val) => val,
            Err(e) => panic!(format!("got the error \"{:?}\" - this is a bug, \
                             please report it at https://github.com/derekdreery/dommer",
                             format_args!($($args),+), e)),
        }
    };
}

/// Allows a message in `expect` that is computed lazily, for performance on the happy path.
macro_rules! expect_opt {
    ($inner:expr, $($args:expr,)+) => {
        expect_opt!($inner, $($args),+)
    };
    ($inner:expr, $($args:expr),+) => {
        match $inner {
            Some(val) => val,
            None => panic!(format!("{} - this is a bug, please report it at \
                                   https://github.com/derekdreery/dommer",
                                   format_args!($($args),+))),
        }
    };
}

/// Generate bindings for a dictionary webidl object (often used as arguments). Name must match a
/// name in web_sys.
macro_rules! dict {
    ($(#[$meta:meta])* $vis:vis struct $name:ident {
        $( $(#[$field_meta:meta])* $field_vis:vis $field:ident: $typ:ty ),*
    }) => {
        $(#[$meta])*
        #[derive(Eq, PartialEq)]
        $vis struct $name {
            $( $(#[$field_meta])* $field_vis $field: Option<$typ> ),*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($field: None),*
                }
            }
        }

        impl $name {
            $(
                $vis fn $field(&mut self, val: $typ) -> &mut Self {
                    self.$field = Some(val);
                    self
                }
            )*

            /// Convert into the corresponding web_sys type.
            pub(crate) fn into_web_sys(&self) -> Option<web_sys::$name> {
                if self == &<Self as Default>::default() {
                    None
                } else {
                    let mut opts = web_sys::$name::new();
                    $(if let Some(val) = self.$field {
                        opts.$field(val);
                    })*
                    Some(opts)
                }
            }
        }
    };
    ($(#[$meta:meta])* $vis:vis struct $name:ident {
        $( $(#[$field_meta:meta])* $field_vis:vis $field:ident: $ty:ty, )*
    }) => {
        dict! {
            $(#[$meta])* $vis struct $name {
                $( $(#[$field_meta])* $field_vis $field: $ty ),*
            }
        }
    };
}
/// This macro works the same as the `println!` from the standard library.
///
/// # Examples
///
/// ```rust,no_run
/// dommer::println!("The number 42 is {}", 42);
/// ```
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::macros::_web_sys::console::log_1(&format!($($args)*).into())
    };
}

/// This macro works the same as the `eprintln!` from the standard library.
///
/// # Examples
///
/// ```rust,no_run
/// dommer::eprintln!("The number 42 is {}", 42);
/// ```
#[macro_export]
macro_rules! eprintln {
    ($($args:tt)*) => {
        $crate::macros::_web_sys::console::error_1(&format!($($args)*).into())
    };
}

/// This macro works like the `console.log` function in javascript.
///
/// # Examples
///
/// ```rust,no_run
/// dommer::log!("a string", 42, true);
/// ```
#[macro_export]
macro_rules! log {
    ($($args:tt,)*) => {
        $crate::log!($($args),*)
    };
    ($($args:tt),*) => {
        let mut arr = $crate::_js_sys::Array::new();
        $(
            arr.push(&$args.into());
        )*
        $crate::macros::_web_sys::console::log(&arr)
    };
}

/// This macro works like the `console.error` function in javascript.
///
/// # Examples
///
/// ```rust,no_run
/// dommer::error!("a string", 42, true);
/// ```
#[macro_export]
macro_rules! error {
    ($($args:tt,)*) => {
        $crate::error!($($args),*)
    };
    ($($args:tt),*) => {
        let mut arr = $crate::_js_sys::Array::new();
        $(
            arr.push(&$args.into());
        )*
        $crate::macros::_web_sys::console::error(&arr)
    };
}
