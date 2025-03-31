use serde::Serialize;
use uuid::Uuid;

use crate::mods::models::user::User;

#[derive(Serialize)]
pub struct OwnerProfile {
    pub user: User, // Composition
    pub creche_id: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub capacity: i32,
}