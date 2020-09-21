use crate::user;

pub fn get() -> rocket::Rocket {
  rocket::ignite().mount("/", routes![user::signup])
}
