use super::models::{Auth, User};
use super::schema::auths;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create(user: &User, connection: &SqliteConnection) -> QueryResult<String> {
    let token = Uuid::new_v4().to_string();
    let token_string = String::from(&token);
    let auth = Auth {
        token: token,
        user_id: String::from(&user.id),
    };

    let result = diesel::insert_into(auths::table)
        .values(auth)
        .execute(connection);

    match result {
        Ok(_) => Ok(String::from(token_string)),
        Err(err) => Err(err),
    }
}
