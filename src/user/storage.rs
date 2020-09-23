use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;

pub fn insert(user: &User, connection: &SqliteConnection) -> QueryResult<usize> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(connection)
}
