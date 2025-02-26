//Routes pour inscriptions (owners et temps)
use actix_web::{post, web, HttpResponse, Responder};
use diesel::*;
use crate::db::DbPool;
use crate::mods::models::forms::RegisterUser;
use crate::mods::models::user::NewUser;
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::{self, hash_password};

#[post("/register")]
async fn register_user(
    data: web::Json<RegisterUser>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    println!("Registering user");
    let conn = &mut pool.get().expect("Erreur connexion DB");
    let conv_hashed_password = hash_password(&data.password);
    let validation_token = security::generate_jwt(&data.email);

    let new_user = NewUser {
        email: data.email.clone(),
        hashed_password: conv_hashed_password,
        role: "pending".to_string(),
        is_validated: Some(false),
        is_profile_validated: Some(false),
    };

    match insert_into(users).values(&new_user).execute(conn) {
        Ok(_) => {
            println!("sending mail");
            // Envoi mail de validation
            crate::mods::utils::email::send_validation_email(&data.email, &validation_token).unwrap();
            HttpResponse::Ok().body("New user registered successfully")},
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
