use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::{user::User,temps::{TempProfile, Temp}};
use crate::mods::utils::schema::{
    temps::dsl::{temps, user_id},
    users::dsl::{email, users},
};
use crate::mods::utils::security;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/profile")]
pub async fn get_profile(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    println!("Getting profile info....");
    match security::extract_token_from_cookie(&req) {
        Ok((mail, _)) => {
            // Connexion à la base de données
            let conn = &mut pool
                .get()
                .expect("Erreur de connexion à la base de données");

            // Chercher les informations de l'utilisateur dans la base de données
            let user_info: User = match users
                .filter(email.eq(mail)) // Filtrer par email
                .select(User::as_select())
                .first::<User>(conn) // Récupérer les données
                .optional()
                .map_err(|e| {
                    ApiError::new(
                        "Erreur lors de la recherche utilisateur",
                        Some(e.to_string()),
                    )
                })? {
                Some(info) => info,
                None => {
                    return Err(ApiError::new(
                        "Utilisateur non trouvé",
                        Some("user_not_found".to_string()),
                    ))
                }
            };
            // Utilisation d'un match pour gérer les différents rôles
            match user_info.role.as_str() {
                "temp" => {
                    // Récupération des infos intérimaires
                    let temp_info: Option<Temp> = match temps
                        .filter(user_id.eq(user_info.id))
                        .first::<Temp>(conn)
                        .optional()
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors de la recherche de profil intérimaire",
                                Some(e.to_string()),
                            )
                        })? {
                        Some(info) => Some(info),
                        None => {
                            return Ok(
                                HttpResponse::NotFound().json("Profil intérimaire non trouvé")
                            )
                        }
                    };

                    let temp_profile = TempProfile { user: user_info, temp: temp_info.unwrap(), availabilities: Vec::new(), conditions: Vec::new(), diplomas: Vec::new(), experiences: Vec::new() };
                    Ok(HttpResponse::Ok().json(temp_profile))
                },
                _ => return Ok(HttpResponse::Ok().json(user_info))
            }
        }
        Err(err) => Err(err), // Renvoie une 401 si le token est invalide
    }
}
