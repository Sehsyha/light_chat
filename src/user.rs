use rocket::http::Status;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct User {
  name: String,
}

#[post("/signup", data = "<user>")]
pub fn signup(user: Json<User>) -> Status {
  println!("{}", user.name);
  Status::Created
}
