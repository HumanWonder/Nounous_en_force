use crate::db::DbPool;
use crate::mods::models::user::User;
use crate::mods::{
    models::apierror::ApiError,
    utils::{schema::users::dsl::*, security},
};
use actix_web::{get, patch, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[get("/admin")]
async fn admin_dashboard(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    println!("Vérification admin...");

    match security::extract_token_from_cookie(&req) {
        Ok((_mail, status)) => {
            println!("status : {}", status);
            // Vérification du rôle admin
            if status != "admin" {
                return Err(ApiError::new(
                    "Accès refusé. Vous n'êtes pas administrateur.",
                    Some("invalid_credentials".to_string()),
                ));
            }
            println!("Accès admin accepté");

            let conn = &mut pool
                .get()
                .expect("Erreur de connexion à la base de données");
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

#[patch("/admin/validate/{user_id}")]
async fn validate_user(
    pool: web::Data<DbPool>, 
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    println!("Trying to validate : {}", user_id);
    // Vérification du token et du statut admin
    match security::extract_token_from_cookie(&req) {
        Ok((_email, status)) => {
            if status != "admin" {
                return Err(ApiError::new(
                    "Accès refusé. Vous n'êtes pas administrateur.",
                    Some("invalid_credentials".to_string()),
                ));
            }
        }
        Err(err) => return Err(err), // Token invalide
    }

    let conn = &mut pool.get().expect("Erreur de connexion à la base de données");

    // Vérifier si l'utilisateur existe
    let target_user = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first::<User>(conn)
        .optional();

    match target_user {
        Ok(Some(_user)) => {
            // Mettre à jour le champ `is_profile_validated` à true
            diesel::update(users.filter(id.eq(user_id)))
                .set(is_profile_validated.eq(true))
                .execute(conn)
                .expect("Erreur lors de la mise à jour");

            Ok(HttpResponse::Ok().json(json!({"message": "Profil validé avec succès"})))
        }
        Ok(None) => Err(ApiError::new(
            "Utilisateur non trouvé",
            Some("not_found".to_string()),
        )),
        Err(_) => Err(ApiError::new(
            "Erreur lors de la récupération de l'utilisateur",
            None,
        )),
    }
}
