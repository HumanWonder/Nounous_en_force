//modèle utilisateur

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::mods::utils::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
    pub role: String,
    pub is_validated: Option<bool>,
}
