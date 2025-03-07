// Routes pour connexion et JWT
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::forms::LoginUser;
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::{self, create_auth_cookie, generate_jwt, verify_password};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use diesel::*;
use uuid::Uuid;

#[post("/login")]
pub async fn login(credentials: web::Json<LoginUser>, pool: web::Data<DbPool>) -> impl Responder {
    let conn = &mut pool.get().expect("Erreur connexion DB");
    println!("Logging in user");

    //Sélectionne la première occurence de l'utilisateur avec l'email correspondant
    let user = match users
        .filter(email.eq(&credentials.email))
        .select((id, hashed_password, email, is_validated))
        .first::<(Uuid, String, String, bool)>(conn)
        .optional()
        .unwrap()
    {
        Some(user) => Some(user),
        None => None,
    };

    println!("Checking credentials : {:?}", user);
    match user {
        Some(user) => {
            //Si is_validated == false
            if !user.3 {
                return Err(ApiError::without_code("Veuillez valider votre mail"));
            };
            if verify_password(&credentials.password, &user.1) {
                let token = generate_jwt(&user.2, Some(user.0), chrono::Duration::hours(2));
                let auth_cookie = security::create_auth_cookie(Some(token));

                Ok(HttpResponse::Ok()
                    .cookie(auth_cookie)
                    .json("Connexion réussie"))
            } else {
                Err(ApiError::new(
                    "Mot de passe incorrect",
                    Some("invalid_credentials".to_string()),
                ))
            }
        }
        None => Err(ApiError::new(
            "Utilisateur non trouvé",
            Some("user_not_found".to_string()),
        )),
    }
}

#[post("/logout")]
pub async fn logout(req: HttpRequest) -> impl Responder {
    match security::extract_token_from_cookie(&req) {
        Ok(_) => {
            // Supprimer le cookie en mettant un Max-Age = 0
            let expired_cookie = create_auth_cookie(None);
            Ok(HttpResponse::Ok()
                .cookie(expired_cookie)
                .body("Déconnexion réussie"))
        }
        Err(_) => Err(ApiError::new(
            "Pas de session active: token invalide",
            Some("invalid_credentials".to_string()),
        )),
    }
}
