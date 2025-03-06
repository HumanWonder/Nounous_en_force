// Routes pour connexion et JWT
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Duration;
use diesel::*;
use uuid::Uuid;
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::forms::LoginUser;
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::{self, generate_jwt, verify_password};

#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginUser>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Erreur connexion DB");
    println!("Logging in user");

    //Sélectionne la première occurence de l'utilisateur avec l'email correspondant
    let user = match users 
        .filter(email.eq(&credentials.email))
        .select((id, hashed_password, email, is_validated))
        .first::<(Uuid, String, String, bool)>(conn)
        .optional().unwrap() {
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
                let token = generate_jwt(&user.2, Some(user.0), Duration::hours(2));
                let auth_cookie = security::create_auth_cookie(token.clone());

                Ok(HttpResponse::Ok().cookie(auth_cookie).json("Connexion réussie"))
                // let response = LoginResponse {
                //     id: user.0,
                //     token,
                // };
                // Ok(HttpResponse::Ok().json(response))
            } else {
                Err(ApiError::new("Mot de passe incorrect", Some("invalid_credentials".to_string())))
            }
        }
        None => Err(ApiError::new("Utilisateur non trouvé", Some("user_not_found".to_string()))),
    }
}
