#![allow(clippy::needless_doctest_main)]
#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

mod callback_server;
pub mod client;
pub mod oauth2;
pub mod util;
