use rocket::get;

#[get("/hello")]
pub fn say_hello() -> &'static str {
  "Hello, world!"
}
