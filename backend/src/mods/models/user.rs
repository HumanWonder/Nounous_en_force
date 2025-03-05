//modèle utilisateur

use crate::mods::utils::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
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
