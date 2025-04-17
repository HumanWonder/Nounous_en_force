// Struct des formulaires d'inscription => format de données reçues par le front
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::mods::utils::schema::{temp_availabilities, temp_conditions, temp_diplomas, temp_documents, temp_experiences, temp_skills, temps,
nursery_description, nursery_responsibles, replacement_needs, nurseries};
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

//Structure qui englobe les tables en une requête
#[derive(Deserialize)]
pub struct TempRequest {
    pub temp_info: TempRegistration,  
    pub availabilities: Vec<TempAvailabilityForm>,  
    pub conditions: Vec<TempConditionForm>,  
    pub diplomas: Vec<TempDiplomaForm>,  
    pub experiences: Vec<TempExperienceForm>,
    pub skills: Vec<TempSkillForm>,
    pub documents: Vec<TempDocumentForm>, 
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temps)]
pub struct TempRegistration {
    #[serde(skip)]
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub phone: String,
    #[serde(skip)]
    pub email: String,
    pub birth_date: Option<NaiveDate>,
    pub has_driver_license: bool,
    pub transport_mode: String,
}
/// 🔹 Disponibilités d'un intérimaire
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_availabilities)]
pub struct TempAvailabilityForm {
    #[serde(skip)]
    pub temp_id: Option<Uuid>,
    pub availability_periods: Option<String>,
    pub time_slots: Option<String>,
    pub geographic_zones: Option<String>,
    pub max_travel_time: Option<String>,
}

/// 🔹 Conditions de travail souhaitées
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_conditions)]
pub struct TempConditionForm {
    #[serde(skip)]
    pub temp_id: Option<Uuid>,
    pub hourly_rate: Option<String>,
    pub contract_types: Option<String>,
    pub auto_entrepreneur: Option<bool>,    //Si pas de contrat de travail cité
}

/// 🔹 Diplômes et certifications
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_diplomas)]
pub struct TempDiplomaForm {
    #[serde(skip)]
    pub temp_id: Option<Uuid>,
    pub main_diploma: String,
    pub other_certifications: Option<String>,
    pub graduation_year: Option<i32>,
    pub school: Option<String>,
}

/// 🔹 Expériences professionnelles
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_experiences)]
pub struct TempExperienceForm {
    #[serde(skip)]
    pub temp_id: Option<Uuid>,
    pub total_experience: Option<String>,
    pub previous_positions: Option<String>,
    pub structure_types: Option<String>,
    pub tasks: Option<String>,
}

// Compétences
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_skills)]
pub struct TempSkillForm {
    #[serde(skip)]
    pub temp_id: Option<Uuid>,
    pub languages: Option<String>,
    pub pedagogies: Option<String>,
    pub special_skills: Option<String>,
    pub special_needs_handling: Option<String>,
}

//Documents fournis
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = temp_documents)]
pub struct TempDocumentForm {
    #[serde(skip)]
    pub temp_id: Option<Uuid>,
    pub motivation_letter: Option<String>,
    pub professional_references: Option<String>,
    pub required_documents: Option<String>,
    pub criminal_record: Option<String>,
    pub diplomas: Option<String>,
}


//Struct pour une requête globale
#[derive(Deserialize)]
pub struct OwnerRequest {
    pub nursery: NewNurseryForm,
    pub description: Option<NurseryDescriptionForm>,
    pub responsible: Option<NurseryResponsibleForm>,
    pub needs: Vec<ReplacementNeedForm>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = nurseries)]
pub struct NewNurseryForm {
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub structure_type: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = nursery_description)]
pub struct NurseryDescriptionForm {
    pub pedagogy: Option<String>,
    pub specificities: Option<String>,
    pub philosophy: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = nursery_responsibles)]
pub struct NurseryResponsibleForm {
    pub first_name: String,
    pub last_name: String,
    pub role: Option<String>,
    pub direct_phone: Option<String>,
    pub direct_email: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = replacement_needs)]
pub struct ReplacementNeedForm {
    pub searched_position: Option<String>,
    pub replacement_reason: Option<String>,
    pub estimated_duration: Option<String>,
    pub available_periods: Option<String>,
    pub hours_per_week: Option<String>,
    pub main_tasks: Option<String>,
    pub required_skills: Option<String>,
    pub suggested_rate: Option<String>,
}