#![deny(clippy::all)]

mod common;
mod exec;
#[cfg(not(windows))]
mod unix;
#[cfg(windows)]
mod windows;
