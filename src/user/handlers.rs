use super::storage;
use crate::auth;
use crate::models::User;
use crate::server::DbConn;
use rocket::http::Status;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub name: String,
}

#[post("/", data = "<user>")]
pub fn create(conn: DbConn, user: Json<UserCreateRequest>) -> Result<Json<TokenResponse>, Status> {
    let id = Uuid::new_v4().to_string();
    let user = User {
        name: user.into_inner().name,
        id,
    };
    match storage::insert(&user, &conn) {
        Ok(_) => match auth::create(&user, &conn) {
            Ok(token) => {
                let token_response = TokenResponse { token };
                Ok(Json(token_response))
            }
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
