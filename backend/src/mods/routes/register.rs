//Routes pour inscriptions (owners et temps)
use crate::db::DbPool;
use crate::mods::models::forms::RegisterUser;
use crate::mods::models::user::NewUser;
use crate::mods::utils::{email, security};
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::hash_password;
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
        is_validated: Some(false),
        is_profile_validated: Some(false),
    };

    match insert_into(users).values(&new_user).execute(conn) {
        Ok(_) => {
            println!("user registered");
            //génération token
            let validation_token = security::generate_jwt(&data.email);

            // Envoi mail de validation
            match email::send_verification_email(&data.email, &validation_token) {
                Ok(_) => HttpResponse::Ok().json("Email envoyé"),
                Err(err) => {
                    eprintln!("Erreur d'envoi d'email: {:?}", err);
                    HttpResponse::InternalServerError().json("Erreur d'envoi d'email")
                }
            }
        }
        Err(err) => {
            eprintln!("Erreur insertion user : {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register user")
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
