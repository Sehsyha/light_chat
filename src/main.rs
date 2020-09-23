#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

pub mod auth;
pub mod models;
pub mod schema;
pub mod server;
pub mod user;

fn main() {
    server::get().launch();
}
