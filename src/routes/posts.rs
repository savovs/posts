use rocket::get;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::db::{pool::Connection, posts, posts::Post};
use crate::helpers::diesel_error_to_status;

#[get("/")]
pub fn get_posts(connection: Connection) -> Result<Json<Vec<Post>>, Status> {
  posts::Post::get_latest(&connection)
    .map(|result| Json(result))
    .map_err(|error| diesel_error_to_status(error))
}
