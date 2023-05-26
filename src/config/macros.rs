macro_rules! destruct {
    (
        $(#[$smeta:meta])*
        $vis:vis struct $struct:ident $(: $derive_default:ident)? {
            $(
                $(#[$kmeta:meta])*
                $key:ident
                    : $type:ty
                    $(= $($default:tt)?)?
            ),* $(,)?
        }
    ) => {
        $(#[$smeta])*
        $vis struct $struct  {
            $(
                $(#[$kmeta])*
                pub $key
                    : $type
            ),*
        }

        impl<'de> ::serde::Deserialize<'de> for $struct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;

                struct MyVisitor;
                impl<'de> serde::de::Visitor<'de> for MyVisitor {
                    type Value = $struct;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a TOML object")
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        $(
                            let mut $key: Option<$type> = None;
                        )*

                        while let Some(key) = map.next_key::<String>()? {
                            match key.as_str() {
                                $(
                                    stringify!($key) => {
                                        if $key.is_some() {
                                            return Err(A::Error::duplicate_field(stringify!($key)));
                                        }
                                        $key = Some(map.next_value()?);
                                    }
                                )*

                                _ => return Err(A::Error::unknown_field(&key, &[ $(stringify!($key)),* ])),
                            }
                        }

                        $(
                            let $key: Result<$type, _> = destruct!(@result_default
                                $key = $($($default)*)?,
                                    else A::Error::missing_field(stringify!($key))
                            );

                            let $key = $key?;
                        )*

                        Ok($struct { $($key),* })
                    }
                }

                deserializer.deserialize_map(MyVisitor)
            }
        }

        destruct!(@if_derive_default if $($derive_default)?
            {
                impl Default for $struct {
                    fn default() -> Self {
                        Self {
                            $(
                                $key: destruct!(@default_value $type, $($($default)*)? )
                            ),*
                        }
                    }
                }
            }
        );
    };

    (@if_derive_default if         {$($then:tt)*}) => {           };
    (@if_derive_default if Default {$($then:tt)*}) => { $($then)* };

    (@default_value $type:ty, Default      ) => { <$type as Default>::default()   };
    (@default_value $type:ty, $default:expr) => { $default.into()                 };
    (@default_value $type:ty,              ) => { compile_error!("Cannot derive default on struct when not all fields have a default value") };

    (@result_default $key:ident = Default      , else $error:expr $(,)?) => { Ok($key.unwrap_or_default())        };
    (@result_default $key:ident = $default:expr, else $error:expr $(,)?) => { Ok($key.unwrap_or($default.into())) };
    (@result_default $key:ident =              , else $error:expr $(,)?) => { $key.ok_or_else(|| $error)          };
}

macro_rules! destructs {
    ( $(
        $(#[$smeta:meta])*
        $vis:vis struct $struct:ident $(: $derive_default:ident)? {
            $(
                $(#[$kmeta:meta])*
                $key:ident
                    : $type:ty
                    $(= $($default:tt)?)?
            ),* $(,)?
        }
    )* ) => {
        $( destruct! {
            $(#[$smeta])*
            $vis struct $struct $(: $derive_default)? {
                $(
                    $(#[$kmeta])*
                    $key: $type $(= $($default)*)?
                ),*
            }
        } )*
    };
}
