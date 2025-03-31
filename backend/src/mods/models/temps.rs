use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable};
use uuid::Uuid;

use crate::mods::models::user::User;
use crate::mods::utils::schema::temps::dsl::*;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Temp {
    pub id: Uuid,
    pub user_id: Uuid, // Relation avec User
    pub full_name: String,
    pub address: String,
    pub phone: String,
    pub birth_date: Option<chrono::NaiveDate>,
    pub driver_license: bool,
    pub transport: String,
    pub motivation: Option<String>,
    pub judicial_record: String,
}

#[derive(Serialize, Deserialize)]
//Struct pour renvoyer toutes les infos d'un coup
pub struct TempProfile {
    pub user: User,
    pub temp: Temp,
    // pub experiences: Vec<Experience>,
    // pub diplomas: Vec<Diploma>,
}
