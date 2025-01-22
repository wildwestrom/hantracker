#[macro_export]
macro_rules! derive_str {
    (
        $(#[$meta:meta])*
        $vis:vis enum $ty:ident {
            $(
                $(#[$att:meta])*
                $var:ident = $s:expr
            ),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $ty {
            $(
                $(#[$att])*
                $var,
            )*
        }

        impl ::core::convert::Into<&'static str> for &$ty {
            fn into(self) -> &'static str {
                match self {
                    $(
                        $ty::$var => $s,
                    )*
                }
            }
        }

        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                f.write_str(self.into())
            }
        }

        impl ::core::str::FromStr for $ty {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $s => $ty::$var,
                    )*
                    _ => return Err(format!("bad {} string: {}", stringify!($ty), s)),
                })
            }
        }
    }
}
