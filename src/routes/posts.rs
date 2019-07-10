use rocket::get;
use crate::db::{Connection};

#[get("/")]
pub fn get_posts(connection: Connection) {}
