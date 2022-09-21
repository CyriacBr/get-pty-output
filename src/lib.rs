#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod common;
mod exec;
// #[cfg(not(windows))]
mod unix;
// #[cfg(windows)]
// mod windows;
