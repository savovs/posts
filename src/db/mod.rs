use diesel::{
  r2d2::{ConnectionManager, Pool, PooledConnection},
  // prelude::*,
  PgConnection,
};
use dotenv;
use rocket::{
  http::Status,
  request::{self, FromRequest},
  Outcome, Request, State,
};
use std::{env, ops::Deref};

pub mod schema;
pub mod traits;
pub mod posts;

// An alias to the type for a pool of a Diesel Pg Connection
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
/// Initialize the database pool.
pub fn connect() -> PgPool {
  dotenv::dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let pool = request.guard::<State<PgPool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(Connection(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

// For the convenience of using an &Connection as an &PgConnection.
impl Deref for Connection {
  type Target = PgConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
