use crate::models::InsertableUser;
use crate::schema::users;
use diesel::prelude::*;

pub fn insert(user: InsertableUser, connection: &SqliteConnection) -> QueryResult<usize> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(connection)
}
