use super::schema::auths;
use super::schema::users;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize)]
pub struct Auth {
    pub user_id: String,
    pub token: String,
}
