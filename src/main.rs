#![feature(proc_macro_hygiene, decl_macro)]

use rocket;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![routes::hello::say_hello])
        .mount("/posts", rocket::routes![routes::posts::get_posts])
        .launch();
}
