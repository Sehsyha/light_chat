use super::user;

#[database("light_chat")]
pub struct DbConn(diesel::SqliteConnection);

pub fn get() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/users", routes![user::handlers::create])
}
