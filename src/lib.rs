#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

extern crate regex;

mod fs_utils;
mod path_utils;
mod replace_text;
