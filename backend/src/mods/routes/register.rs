//Routes pour inscriptions (owners et temps)
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::forms::RegisterUser;
use crate::mods::models::user::NewUser;
use crate::mods::utils::{email, security};
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::hash_password;
use chrono::Duration;
use actix_web::{post, web, HttpResponse, Responder};
use diesel::*;

#[post("/register")]
async fn register_user(data: web::Json<RegisterUser>, pool: web::Data<DbPool>) -> impl Responder {
    println!("Registering user");
    let conn = &mut pool.get().expect("Erreur connexion DB");
    let conv_hashed_password = hash_password(&data.password);

    let new_user = NewUser {
        email: data.email.clone(),
        hashed_password: conv_hashed_password,
        role: "pending".to_string(),
        is_validated: false,
        is_profile_validated: false,
    };

    match insert_into(users).values(&new_user).execute(conn) {
        Ok(_) => {
            println!("user registered");
            //génération token
            let validation_token = security::generate_jwt(&data.email, None, Duration::minutes(15));

            // Envoi mail de validation
            match email::send_verification_email(&data.email, &validation_token) {
                Ok(_) => Ok(HttpResponse::Ok().json("Email envoyé")),
                Err(err) => {
                    println!("Erreur d'envoi d'email: {:?}", err);
                    Err(ApiError::new("Erreur dans l'envoi de l'email", None))
                }
            }
        }
        Err(err) => {
            println!("Erreur insertion user : {:?}", err);
            Err(ApiError::new("Failed to register user", Some("db_insert_failed".to_string())))
        }
    }
}

// #[post("/register/temp")]
// async fn register_temp(
//     data: web::Json<RegisterTemp>,
//     pool: web::Data<DbPool>,
// ) -> impl Responder {
//     let conn = &mut pool.get().expect("Erreur connexion DB");
//     let conv_hashed_password = hash_password(&data.password);

//     let new_user = NewUser {
//         email: data.email.clone(),
//         hashed_password: conv_hashed_password,
//         role: "temp".to_string(),
//         is_validated: Some(false),
//     };

//     match diesel::insert_into(users).values(&new_user).execute(conn) {
//         Ok(_) => HttpResponse::Ok().body("Temp registered successfully"),
//         Err(err) => {
//             eprintln!("Erreur insertion Temp : {:?}", err);
//             HttpResponse::InternalServerError().body("Failed to register Temp")
//         }
//     }
// }
