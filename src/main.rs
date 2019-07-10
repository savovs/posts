#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate serde;

mod db;
mod routes;
mod helpers;

fn main() {
    rocket::ignite()
        .manage(db::pool::connect())
        .mount("/", rocket::routes![routes::hello::say_hello])
        .mount("/posts", rocket::routes![routes::posts::get_posts])
        .launch();
}
