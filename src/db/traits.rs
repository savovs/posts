use diesel::PgConnection;

pub trait Crud {
  fn create(&self, connection: &PgConnection);
  fn get_by_id(id: i32, connection: &PgConnection) -> Self
  where
    Self: Sized;
}
