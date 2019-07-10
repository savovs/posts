use diesel::PgConnection;

pub trait CRUD {
  fn create(&self, connection: &PgConnection);
  fn read(id: i32, connection: &PgConnection) -> Self where Self: Sized;
}
