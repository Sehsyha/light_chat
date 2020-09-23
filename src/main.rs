#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use diesel;

#[database("light_chat")]
struct DbConn(diesel::SqliteConnection);

mod server;
mod user;

fn main() {
    server::get().launch();
}
