use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable};
use uuid::Uuid;
use chrono::NaiveDate;

use crate::mods::models::user::User;
use crate::mods::utils::schema::{temps, temp_availabilities, temp_conditions, temp_diplomas, temp_experiences};

#[derive(Queryable, Selectable, Serialize, Deserialize, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Temp {
    pub id: Uuid,
    pub user_id: Uuid, // Relation avec User
    pub full_name: String,
    pub address: String,
    pub phone: String,
    pub birth_date: Option<NaiveDate>,
    pub driver_license: bool,
    pub transport: String,
    pub motivation: Option<String>,
    pub judicial_record: String,
}

// ✅ Disponibilités
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempAvailabilitie {
    pub id: Uuid,
    pub temp_id: Uuid, // Relation avec Temp
    pub available_periods: String, 
    pub work_hours: String,
    pub preferred_locations: String,
    pub max_travel_time: String,
}

// ✅ Conditions de travail
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempCondition {
    pub id: Uuid,
    pub temp_id: Uuid, // Relation avec Temp
    pub hourly_rate: String, 
    pub contract_types: String, 
    pub self_employment: bool,
}

// ✅ Diplômes et certifications
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempDiploma {
    pub id: Uuid,
    pub temp_id: Uuid, // Relation avec Temp
    pub diploma_name: String,
    pub other_certifications: Option<String>,
    pub year_obtained: i32,
    pub institution: String,
}

// ✅ Expériences professionnelles
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempExperience {
    pub id: Uuid,
    pub temp_id: Uuid, // Relation avec Temp
    pub total_experience: String,
    pub previous_jobs: String,
    pub structure_types: String,
    pub tasks: String,
}

// ✅ Struct complète pour renvoyer le profil d’un intérimaire
#[derive(Serialize, Deserialize)]
pub struct TempProfile {
    pub user: User,
    pub temp: Temp,
    pub availabilities: Vec<TempAvailabilitie>,
    pub conditions: Vec<TempCondition>,
    pub diplomas: Vec<TempDiploma>,
    pub experiences: Vec<TempExperience>,
}
