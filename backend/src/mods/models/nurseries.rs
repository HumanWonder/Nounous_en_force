use serde::{Serialize, Deserialize};
use diesel::prelude::Queryable;
use uuid::Uuid;

use crate::mods::models::user::User;

#[derive(Serialize)]
// #[diesel(table_name = creche_responsables)]
pub struct OwnerProfile {
    pub user: User, // Composition
    pub creche_id: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub capacity: i32,
}

//Informations de la créche
#[derive(Queryable, Serialize, Deserialize)]
pub struct Nursery {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub structure_type: Option<String>,
}

//Description de la créche
#[derive(Queryable, Serialize, Deserialize)]
pub struct NurseryDescription {
    pub id: Uuid,
    pub nursery_id: Option<Uuid>,
    pub pedagogy: Option<String>,
    pub specificities: Option<String>,
    pub philosophy: Option<String>,
}

//Informations sur le/la principal responsable de la créche
#[derive(Queryable, Serialize, Deserialize)]
pub struct NurseryResponsible {
    pub id: Uuid,
    pub nursery_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub role: Option<String>,
    pub direct_phone: Option<String>,
    pub direct_email: Option<String>,
}

//Description des besoins de remplacement
#[derive(Queryable, Serialize, Deserialize)]
pub struct ReplacementNeed {
    pub id: Uuid,
    pub nursery_id: Option<Uuid>,
    pub searched_position: Option<String>,
    pub replacement_reason: Option<String>,
    pub estimated_duration: Option<String>,
    pub available_periods: Option<String>,
    pub hours_per_week: Option<String>,
    pub main_tasks: Option<String>,
    pub required_skills: Option<String>,
    pub suggested_rate: Option<String>,
}