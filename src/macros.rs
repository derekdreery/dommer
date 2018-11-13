//! Internal macros

/// Allows a message in `expect` that is computed lazily, for performance on the happy path.
macro_rules! expect {
    ($inner:expr, $($args:expr,)+) => {
        expect!($inner, $($args),+)
    };
    ($inner:expr, $($args:expr),+) => {
        match $inner {
            Ok(val) => val,
            Err(e) => panic!(format!("{}: {:?}", format_args!($($args),+), e)),
        }
    };
    ($inner:expr) => {
        match $inner {
            Ok(val) => val,
            Err(e) => panic!(format!("{:?}", e))
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
            None => panic!(format!($($args),+)),
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
