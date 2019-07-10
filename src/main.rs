#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod db;
mod routes;

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![routes::hello::say_hello])
        .mount("/posts", rocket::routes![routes::posts::get_posts])
        .launch();
}
