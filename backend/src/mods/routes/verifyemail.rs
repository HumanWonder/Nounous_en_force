use std::collections::HashMap;

use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::utils::schema::users;
use crate::mods::utils::security;


#[post("/verify_email")]
pub async fn verify_email(data: web::Json<HashMap<String, String>>, pool: web::Data<DbPool>) -> impl Responder {
    println!("Requête pour la vérification email reçue.");
    let token = match data.get("token") {
        Some(t) => t,
        // None => return HttpResponse::BadRequest().json("Token manquant"),
        None => return Err(ApiError::new("Token manquant", Some("invalid_credentials".to_string()))),
    };
    match security::verify_jwt(token) {
        Ok(email) => {
            let conn = &mut pool.get().expect("Erreur connexion DB");

            // Mise à jour de is_validated à true
            match diesel::update(users::table.filter(users::email.eq(email)))
                .set(users::is_validated.eq(true))
                .execute(conn)
            {
                Ok(_) => {
                    println!("Email vérifié avec succès ! Statut modifié");
                    Ok(HttpResponse::Ok().json(json!({
                        "success": true,
                        "message": "Votre email a été vérifié avec succès !"
                    })))
                },
                Err(err) => {
                    eprintln!("Erreur mise à jour user : {:?}", err);
                    Err(ApiError::new("Erreur mise à jour statut user", Some("db_update_failed".to_string())))
                }
            }
        }
        Err(_) => {
            println!("Token invalide ou expiré");
            Err(ApiError::new("Token invalide ou expiré.", Some("invalid_credentials".to_string())))
        },
    }
}
