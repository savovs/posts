use diesel::{prelude::*, PgConnection};
use std::fmt;

use super::schema::posts;
use super::traits::CRUD;

#[derive(Queryable, Identifiable, Insertable)]
#[table_name = "posts"]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
  pub views: i32,
}

impl CRUD for Post {
  fn create(&self, connection: &PgConnection) {
    diesel::insert_into(posts::table)
      .values(self)
      .execute(connection)
      .expect("Error inserting new post");
  }

  fn read(id: i32, connection: &PgConnection) -> Post {
    posts::table
      .find(id)
      .first::<Post>(connection)
      .expect("Error finding post by id")
  }
}

impl fmt::Debug for Post {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Post {{ title: {}}}", self.title)
  }
}
