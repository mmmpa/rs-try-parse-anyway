#![allow(warnings)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use try_parse_anyway_proc_macro::TryParseAnything;
use try_parse_anyway_type::{TryParseAnywayError, TryParseAnywayErrorItem};

fn main() {
    #[derive(Default, Debug, TryParseAnything)]
    pub struct A {
        a: u8,
    }

    #[derive(Default, Debug, TryParseAnything)]
    pub struct B {
        pub(crate) a: u8,
        pub b: String,
    }

    #[derive(Default, Debug, TryParseAnything)]
    pub struct C {
        pub a: String,
        pub b: String,
        pub c: String,
    }

    let re = C::try_parse_anything("abc".as_bytes());

    println!("{:?}", re);

    let re = C::try_parse_anything(r#"{ "a": "a" }"#.as_bytes());

    println!("{:?}", re.unwrap_err());

    let re = C::try_parse_anything(r#"{ "a": "a", "b": "b", "c": "c" }"#.as_bytes());

    println!("{:?}", re.unwrap());

    #[derive(Default, Debug, TryParseAnything)]
    pub struct Types {
        pub a: Vec<String>,
        pub b: Option<usize>,
        pub c: String,
    }

    let re = Types::try_parse_anything(r#"{ "a": ["a"], "b": 1, "c": "c" }"#.as_bytes());

    println!("{:?}", re.unwrap());
}

