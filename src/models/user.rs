use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Debug, Queryable, diesel::Identifiable)]
#[primary_key(id)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub pass: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub email: Option<String>,
    pub pass: Option<String>,
}