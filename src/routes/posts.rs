use rocket::get;
use diesel;
use diesel::prelude::*;

#[get("/")]
pub fn get_posts() {}
