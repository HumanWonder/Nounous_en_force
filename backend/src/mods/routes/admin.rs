use crate::db::DbPool;
use crate::mods::{
    models::apierror::ApiError,
    utils::{schema::users::dsl::*, security},
};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

#[get("/admin")]
async fn admin_dashboard(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    println!("Accès au dashboard admin, vérification du user...");

    match security::extract_token_from_cookie(&req) {
        Ok(mail) => {
            let conn = &mut pool.get().expect("Erreur connexion DB");

            let user_role = match users
                .filter(email.eq(mail))
                .select(role)
                .first::<String>(conn)
                .optional()
                .unwrap()
            {
                Some(role_db) => role_db,
                None => {
                    return Err(ApiError::new(
                        "Utilisateur non trouvé",
                        Some("user_not_found".to_string()),
                    ))
                }
            };

            // Vérification du rôle admin
            if user_role != "admin" {
                return Err(ApiError::new(
                    "Accès refusé. Vous n'êtes pas administrateur.",
                    Some("invalid_credentials".to_string()),
                ));
            }
            println!("Accès admin accepté");

            // Récupération des utilisateurs
            let results = users
                .filter(is_profile_validated.eq(false)) //Ne chercher que les profils non validés
                .select((id, email, role, is_profile_validated))
                .load::<(Uuid, String, String, bool)>(conn);

            match results {
                Ok(user_list) => Ok(HttpResponse::Ok().json(user_list)),
                Err(_) => Err(ApiError::new(
                    "Impossible de récupérer les utilisateurs.",
                    None,
                )),
            }
        }
        Err(err) => Err(err), //Token invalide
    }
}
