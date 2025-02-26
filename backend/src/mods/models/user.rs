//modèle utilisateur

use crate::mods::utils::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub is_validated: Option<bool>,
    pub is_profile_validated: Option<bool>, // Validation par admin
    pub role: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
    pub role: String,
    pub is_validated: Option<bool>, //Validation email
    pub is_profile_validated: Option<bool>, // Validation par admin
}
