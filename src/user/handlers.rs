use super::storage;
use crate::models::InsertableUser;
use crate::server::DbConn;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/", data = "<user>")]
pub fn create(conn: DbConn, user: Json<InsertableUser>) -> Status {
    match storage::insert(user.into_inner(), &conn) {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}
