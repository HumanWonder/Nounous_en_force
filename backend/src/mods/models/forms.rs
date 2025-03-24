// Struct des formulaires d'inscription
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::mods::utils::schema::temps;
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

// #[derive(Deserialize, Serialize)]
// pub struct LoginResponse {
//     pub id: Uuid,
//     pub token: String,
// }
#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// #[derive(Deserialize)]
// pub struct RegisterOwner {
//     pub email: String,
//     pub password: String,
//     pub last_name: String,
//     pub first_name: String,
//     pub job_position: Option<String>,
//     pub tel_number: Option<String>,
//     pub address: Option<String>,
// }

// id -> Uuid,
//         user_id -> Uuid,
//         full_name -> Text,
//         address -> Text,
//         phone -> Text,
//         email -> Text,
//         birth_date -> Nullable<Date>,
//         driver_license -> Bool,
//         transport -> Text,
//         motivation -> Nullable<Text>,
//         judicial_record -> Text,
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temps)]
pub struct TempRegistration {
    pub user_id: Uuid,
    pub full_name: String,
    pub address: String,
    pub phone: String,
    pub birth_date: Option<NaiveDate>,  // Nullable si la date de naissance est optionnelle
    pub driver_license: bool,
    pub transport: String,
    pub motivation: Option<String>,  // Nullable pour la motivation
    pub judicial_record: String,
}
