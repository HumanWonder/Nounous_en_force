use serde::{Serialize, Deserialize};
use diesel::prelude::{Queryable, Insertable, Identifiable, Associations};
use uuid::Uuid;

use crate::mods::utils::schema::{
    nurseries,
    nursery_description,
    nursery_responsibles,
    replacement_needs,
};

#[derive(Serialize)]
// #[diesel(table_name = creche_responsables)]
pub struct OwnerProfile {
    pub nursery: Vec<Nursery>,
    pub responsible: Vec<NurseryResponsible>,
    pub description: Vec<NurseryDescription>,
    pub needs: Vec<ReplacementNeed>,
}

//Informations de la créche
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = nurseries)]
pub struct Nursery {
    pub id: Uuid,
    //Lien entre le user connecté et les crèches dont il a accès
    pub user_id: Option<Uuid>,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub structure_type: Option<String>,
}

//Description de la créche
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[diesel(belongs_to(Nursery))]
#[diesel(table_name = nursery_description)]
pub struct NurseryDescription {
    pub id: Uuid,
    pub nursery_id: Option<Uuid>,
    pub pedagogy: Option<String>,
    pub specificities: Option<String>,
    pub philosophy: Option<String>,
}

//Informations sur le/la principal responsable de la créche
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[diesel(belongs_to(Nursery))]
#[diesel(table_name = nursery_responsibles)]
//Liste des responsables (drh, directrice, adjoint,etc.)
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
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[diesel(belongs_to(Nursery))]
#[diesel(table_name = replacement_needs)]
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