#![allow(warnings)]

use try_parse_anyway::prelude::*;

fn main() {
    #[derive(Default, Debug, TryParseAnyway)]
    pub struct A {
        a: u8,
    }

    #[derive(Default, Debug, TryParseAnyway)]
    pub struct B {
        pub(crate) a: u8,
        pub b: String,
    }

    #[derive(Default, Debug, TryParseAnyway)]
    pub struct C {
        pub a: String,
        pub b: String,
        pub c: String,
    }

    let re = C::try_from_slice_anyway("abc".as_bytes());

    println!("{:?}", re);

    let re = C::try_from_str_anyway(r#"{ "a": "a" }"#);

    println!("{:?}", re.unwrap_err());

    let re = C::try_from_value_anyway(
        serde_json::from_str(r#"{ "a": "a", "b": "b", "c": "c" }"#).unwrap(),
    );

    println!("{:?}", re.unwrap());

    #[derive(Default, Debug, TryParseAnyway)]
    pub struct Types {
        pub a: Vec<String>,
        pub b: Option<usize>,
        pub c: String,
    }

    let re = Types::try_from_slice_anyway(r#"{ "a": ["a"], "b": 1, "c": "c" }"#.as_bytes());

    println!("{:?}", re.unwrap());
}
