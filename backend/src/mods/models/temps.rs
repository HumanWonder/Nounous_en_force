use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable};
use uuid::Uuid;
use chrono::NaiveDate;

use crate::mods::utils::schema::{temps, temp_availabilities, temp_conditions, temp_diplomas, temp_experiences};

// Struct complète pour renvoyer le profil d’un intérimaire
#[derive(Serialize, Deserialize)]
pub struct TempProfile {
    pub temp: Temp,
    pub availabilities: Vec<TempAvailabilitie>,
    pub conditions: Vec<TempCondition>,
    pub diplomas: Vec<TempDiploma>,
    pub experiences: Vec<TempExperience>,
}

// Informations personnelles
#[derive(Queryable, Selectable, Serialize, Deserialize, Default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Temp {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub birth_date: Option<NaiveDate>,
    pub has_driver_license: bool,
    pub transport_mode: String,
}

// Disponibilités
#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempAvailabilitie {
    pub id: Uuid,
    pub temp_id: Option<Uuid>,
    pub availability_periods: Option<String>,
    pub time_slots: Option<String>,
    pub geographic_zones: Option<String>,
    pub max_travel_time: Option<String>,
}
// Conditions de travail
#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempCondition {
    pub id: Uuid,
    pub temp_id: Option<Uuid>,
    pub hourly_rate: Option<String>,
    pub contract_types: Option<String>,
    pub auto_entrepreneur: Option<bool>,
}

// Diplômes et certifications
#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempDiploma {
    pub id: Uuid,
    pub temp_id: Option<Uuid>,
    pub main_diploma: String,
    pub other_certifications: Option<String>,
    pub graduation_year: Option<i32>,
    pub school: Option<String>,
}

// Expériences professionnelles
#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Temp))]
pub struct TempExperience {
    pub id: Uuid,
    pub temp_id: Option<Uuid>,
    pub total_experience: Option<String>,
    pub previous_positions: Option<String>,
    pub structure_types: Option<String>,
    pub tasks: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(belongs_to(Temp))]
pub struct TempSkill {
    pub id: Uuid,
    pub temp_id: Option<Uuid>,
    pub languages: Option<String>,
    pub pedagogies: Option<String>,
    pub special_skills: Option<String>,
    pub special_needs_handling: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(belongs_to(Temp))]
pub struct TempDocument {
    pub id: Uuid,
    pub temp_id: Option<Uuid>,
    pub motivation_letter: Option<String>,
    pub professional_references: Option<String>,
    pub required_documents: Option<String>,
    pub criminal_record: Option<String>,
    pub diplomas: Option<String>,
}
