// TODO: replace with derive_more impl
macro_rules! common_header_deref {
    ($from:ty => $to:ty) => {
        impl ::core::ops::Deref for $from {
            type Target = $to;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $from {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// Sets up a test module with some useful imports for use with [`common_header_test!`].
macro_rules! common_header_test_module {
    ($id:ident, $tm:ident{$($tf:item)*}) => {
        #[cfg(test)]
        mod $tm {
            #![allow(unused_imports)]

            use std::str;
            use actix_http::http::Method;
            use mime::*;
            use $crate::http::header::*;
            use super::{$id as HeaderField, *};
            $($tf)*
        }
    }
}

#[cfg(test)]
macro_rules! common_header_test {
    ($id:ident, $raw:expr) => {
        #[test]
        fn $id() {
            use actix_http::test;

            let raw = $raw;
            let a: Vec<Vec<u8>> = raw.iter().map(|x| x.to_vec()).collect();

            let mut req = test::TestRequest::default();

            for item in a {
                req = req.insert_header((HeaderField::name(), item)).take();
            }

            let req = req.finish();
            let value = HeaderField::parse(&req);

            let result = format!("{}", value.unwrap());
            let expected = String::from_utf8(raw[0].to_vec()).unwrap();

            let result_cmp: Vec<String> = result
                .to_ascii_lowercase()
                .split(' ')
                .map(|x| x.to_owned())
                .collect();
            let expected_cmp: Vec<String> = expected
                .to_ascii_lowercase()
                .split(' ')
                .map(|x| x.to_owned())
                .collect();

            assert_eq!(result_cmp.concat(), expected_cmp.concat());
        }
    };

    ($id:ident, $raw:expr, $exp:expr) => {
        #[test]
        fn $id() {
            use actix_http::test;

            let a: Vec<Vec<u8>> = $raw.iter().map(|x| x.to_vec()).collect();
            let mut req = test::TestRequest::default();
            for item in a {
                req.insert_header((HeaderField::name(), item));
            }
            let req = req.finish();
            let val = HeaderField::parse(&req);
            let exp: Option<HeaderField> = $exp;

            println!("req: {:?}", &req);
            println!("val: {:?}", &val);
            println!("exp: {:?}", &exp);

            // test parsing
            assert_eq!(val.ok(), exp);

            // test formatting
            if let Some(exp) = exp {
                let raw = &($raw)[..];
                let mut iter = raw.iter().map(|b| str::from_utf8(&b[..]).unwrap());
                let mut joined = String::new();
                if let Some(s) = iter.next() {
                    joined.push_str(s);
                    for s in iter {
                        joined.push_str(", ");
                        joined.push_str(s);
                    }
                }
                assert_eq!(format!("{}", exp), joined);
            }
        }
    };
}

macro_rules! common_header {
    // TODO: these docs are wrong, there's no $n or $nn
    // $a:meta: Attributes associated with the header item (usually docs)
    // $id:ident: Identifier of the header
    // $n:expr: Lowercase name of the header
    // $nn:expr: Nice name of the header

    // List header, zero or more items
    ($(#[$a:meta])*($id:ident, $name:expr) => ($item:ty)*) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub Vec<$item>);
        crate::http::header::common_header_deref!($id => Vec<$item>);
        impl $crate::http::header::Header for $id {
            #[inline]
            fn name() -> $crate::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: $crate::HttpMessage>(msg: &M) -> Result<Self, $crate::error::ParseError> {
                let headers = msg.headers().get_all(Self::name());
                $crate::http::header::from_comma_delimited(headers).map($id)
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                $crate::http::header::fmt_comma_delimited(f, &self.0[..])
            }
        }

        impl $crate::http::header::IntoHeaderValue for $id {
            type Error = $crate::http::header::InvalidHeaderValue;

            fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
                use ::core::fmt::Write;
                let mut writer = $crate::http::header::Writer::new();
                let _ = write!(&mut writer, "{}", self);
                $crate::http::header::HeaderValue::from_maybe_shared(writer.take())
            }
        }
    };

    // List header, one or more items
    ($(#[$a:meta])*($id:ident, $name:expr) => ($item:ty)+) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub Vec<$item>);

        crate::http::header::common_header_deref!($id => Vec<$item>);

        impl $crate::http::header::Header for $id {
            #[inline]
            fn name() -> $crate::http::header::HeaderName {
                $name
            }
            #[inline]
            fn parse<M: $crate::HttpMessage>(msg: &M) -> Result<Self, $crate::error::ParseError> {
                $crate::http::header::from_comma_delimited(
                    msg.headers().get_all(Self::name())).map($id)
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                $crate::http::header::fmt_comma_delimited(f, &self.0[..])
            }
        }

        impl $crate::http::header::IntoHeaderValue for $id {
            type Error = $crate::http::header::InvalidHeaderValue;

            fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
                use ::core::fmt::Write;
                let mut writer = $crate::http::header::Writer::new();
                let _ = write!(&mut writer, "{}", self);
                $crate::http::header::HeaderValue::from_maybe_shared(writer.take())
            }
        }
    };

    // Single value header
    ($(#[$a:meta])*($id:ident, $name:expr) => [$value:ty]) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub $value);

        crate::http::header::common_header_deref!($id => $value);

        impl $crate::http::header::Header for $id {
            #[inline]
            fn name() -> $crate::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: $crate::HttpMessage>(msg: &M) -> Result<Self, $crate::error::ParseError> {
                let header = msg.headers().get(Self::name());
                $crate::http::header::from_one_raw_str(header).map($id)
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl $crate::http::header::IntoHeaderValue for $id {
            type Error = $crate::http::header::InvalidHeaderValue;

            fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
                self.0.try_into_value()
            }
        }
    };

    // List header, one or more items with "*" option
    ($(#[$a:meta])*($id:ident, $name:expr) => {Any / ($item:ty)+}) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub enum $id {
            /// Any value is a match
            Any,
            /// Only the listed items are a match
            Items(Vec<$item>),
        }

        impl $crate::http::header::Header for $id {
            #[inline]
            fn name() -> $crate::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: $crate::HttpMessage>(msg: &M) -> Result<Self, $crate::error::ParseError> {
                let is_any = msg
                    .headers()
                    .get(Self::name())
                    .and_then(|hdr| hdr.to_str().ok())
                    .map(|hdr| hdr.trim() == "*");

                if let Some(true) = is_any {
                    Ok($id::Any)
                } else {
                    let headers = msg.headers().get_all(Self::name());
                    Ok($id::Items($crate::http::header::from_comma_delimited(headers)?))
                }
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match *self {
                    $id::Any => f.write_str("*"),
                    $id::Items(ref fields) =>
                        $crate::http::header::fmt_comma_delimited(f, &fields[..])
                }
            }
        }

        impl $crate::http::header::IntoHeaderValue for $id {
            type Error = $crate::http::header::InvalidHeaderValue;

            fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
                use ::core::fmt::Write;
                let mut writer = $crate::http::header::Writer::new();
                let _ = write!(&mut writer, "{}", self);
                $crate::http::header::HeaderValue::from_maybe_shared(writer.take())
            }
        }
    };

    // optional test module
    ($(#[$a:meta])*($id:ident, $name:expr) => ($item:ty)* $tm:ident{$($tf:item)*}) => {
        crate::http::header::common_header! {
            $(#[$a])*
            ($id, $name) => ($item)*
        }

        crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)+ $tm:ident{$($tf:item)*}) => {
        crate::http::header::common_header! {
            $(#[$a])*
            ($id, $n) => ($item)+
        }

        crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $name:expr) => [$item:ty] $tm:ident{$($tf:item)*}) => {
        crate::http::header::common_header! {
            $(#[$a])* ($id, $name) => [$item]
        }

        crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $name:expr) => {Any / ($item:ty)+} $tm:ident{$($tf:item)*}) => {
        crate::http::header::common_header! {
            $(#[$a])*
            ($id, $name) => {Any / ($item)+}
        }

        crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
    };
}

pub(crate) use {common_header, common_header_deref, common_header_test_module};

#[cfg(test)]
pub(crate) use common_header_test;
