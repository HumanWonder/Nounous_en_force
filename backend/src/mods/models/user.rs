//modèle utilisateur

use crate::mods::utils::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::mods::models::{
    nurseries::OwnerProfile,
    temps::TempProfile,
};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub is_validated: bool,
    pub is_profile_validated: bool, // Validation par admin
    pub role: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub rejection_reason: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
    pub role: String,
    pub is_validated: bool,         //Validation email
    pub is_profile_validated: bool, // Validation par admin
}

#[derive(Serialize)]
#[serde(tag = "role_data", rename_all = "snake_case")]
pub enum FullProfileData {
    Temp {
        user: User,
        temp: TempProfile
    },
    Owner {
        user: User,
        owner_info: OwnerProfile,
    },
    Basic {
        user: User,
    },
}
