// Struct des formulaires d'inscription
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::mods::utils::schema::{temps, temp_availabilities, temp_experiences, temp_diplomas, temp_conditions};
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

//Structure qui englobe les tables en une requête
#[derive(Deserialize)]
pub struct TempRequest {
    pub temp_info: TempRegistration,  
    pub availabilities: Vec<TempAvailabilityForm>,  
    pub work_hours: Vec<TempConditionForm>,  
    pub documents: Vec<TempDiplomaForm>,  
    pub experiences: Vec<TempExperienceForm>,  
}

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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temps)]
pub struct TempRegistration {
    #[serde(skip)]
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
/// 🔹 Disponibilités d'un intérimaire
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_availabilities)]
pub struct TempAvailabilityForm {
    #[serde(skip)]
    pub temp_id: Uuid,
    pub available_periods: String,  // JSON ou texte structuré (ex: "Matin, Après-midi")
    pub work_hours: String,         // JSON ou texte structuré (ex: "08:00-12:00, 14:00-18:00")
    pub preferred_locations: String, // JSON ou liste séparée par des virgules
    pub max_travel_time: String,    // En minutes ou format texte (ex: "30 min")
}

/// 🔹 Conditions de travail souhaitées
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_conditions)]
pub struct TempConditionForm {
    #[serde(skip)]
    pub temp_id: Uuid,
    pub hourly_rate: String,   // Peut être converti en `f64` si nécessaire
    pub contract_types: String, // Liste JSON ou séparée par des virgules (ex: "CDI, CDD, Intérim")
    pub self_employment: bool,  // Indique si le travailleur accepte d'être auto-entrepreneur
}

/// 🔹 Diplômes et certifications
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_diplomas)]
pub struct TempDiplomaForm {
    #[serde(skip)]
    pub temp_id: Uuid,
    pub diploma_name: String,
    pub other_certifications: Option<String>, // Peut être NULL si pas d'autres certifications
    pub year_obtained: i32,
    pub institution: String,
}

/// 🔹 Expériences professionnelles
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_experiences)]
pub struct TempExperienceForm {
    #[serde(skip)]
    pub temp_id: Uuid,
    pub total_experience: String, // Nombre d'années en texte (ex: "5 ans")
    pub previous_jobs: String,  // Liste JSON ou séparée par des virgules (ex: "Crèche, Maternelle")
    pub structure_types: String, // Ex: "Publique, Privée"
    pub tasks: String,  // Liste des tâches effectuées (ex: "Changer les couches, donner à manger")
}