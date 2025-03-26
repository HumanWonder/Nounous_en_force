use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
// NOTE ! 
// Créer user avec rôle admin pour voir toutes les demandes et pouvoir en valider ou refuser
#[get("/profile")]
pub async fn get_profile(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    println!("Getting profile info....");
    match security::extract_token_from_cookie(&req) {
        Ok(mail) => {
            // Connexion à la base de données
            let conn = &mut pool
                .get()
                .expect("Erreur de connexion à la base de données");
            
            // Chercher les informations de l'utilisateur dans la base de données
            let user_info = match users
                .filter(email.eq(mail)) // Filtrer par email
                .select((id, email, role)) // Sélectionner les colonnes nécessaires
                .first::<(Uuid, String, String)>(conn) // Récupérer les données
                .optional()
                .unwrap()
            {
                Some(info) => info, // Si trouvé, on retourne les données
                None => {
                    return Err(ApiError::new(
                        "Utilisateur non trouvé",
                        Some("user_not_found".to_string()),
                    ))
                }
            };
            // Renvoi des informations de l'utilisateur
            Ok(HttpResponse::Ok().json(user_info)) // Retourner les données sous forme JSON
        }
        Err(err) => Err(err), // Renvoie une 401 si le token est invalide
    }
}
