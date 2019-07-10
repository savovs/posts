use std::fmt;
use serde::{Serialize, Deserialize};
use diesel::{prelude::*, PgConnection};

use super::schema::posts;
use super::traits::Crud;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
  pub views: i32,
}

impl Crud for Post {
  fn create(&self, connection: &PgConnection) {
    diesel::insert_into(posts::table)
      .values(self)
      .execute(connection)
      .expect("Error inserting new post");
  }

  fn get_by_id(id: i32, connection: &PgConnection) -> Post {
    posts::table
      .find(id)
      .first::<Post>(connection)
      .expect("Error finding post by id")
  }
}

impl Post {
  pub fn get_latest(connection: &PgConnection) -> QueryResult<Vec<Post>> {
    posts::table.limit(5).load::<Post>(connection)
  }
}

impl fmt::Debug for Post {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Post {{ title: {}}}", self.title)
  }
}
