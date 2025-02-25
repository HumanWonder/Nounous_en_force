//Routes pour inscriptions (owners et temps)
use actix_web::{post, web, HttpResponse, Responder};
use diesel::*;
use crate::db::DbPool;
use crate::mods::models::forms::{RegisterOwner, RegisterTemp};
use crate::mods::models::user::NewUser;
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::hash_password;

#[post("/register/owner")]
async fn register_owner(
    data: web::Json<RegisterOwner>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Erreur connexion DB");
    let conv_hashed_password = hash_password(&data.password);

    let new_user = NewUser {
        email: data.email.clone(),
        hashed_password: conv_hashed_password,
        role: "owner".to_string(),
        is_validated: Some(false),
    };

    match insert_into(users).values(&new_user).execute(conn) {
        Ok(_) => HttpResponse::Ok().body("Owner registered successfully"),
        Err(err) => {
            eprintln!("Erreur insertion Owner : {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register Owner")
        }
    }
}

#[post("/register/temp")]
async fn register_temp(
    data: web::Json<RegisterTemp>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Erreur connexion DB");
    let conv_hashed_password = hash_password(&data.password);

    let new_user = NewUser {
        email: data.email.clone(),
        hashed_password: conv_hashed_password,
        role: "temp".to_string(),
        is_validated: Some(false),
    };

    match diesel::insert_into(users).values(&new_user).execute(conn) {
        Ok(_) => HttpResponse::Ok().body("Temp registered successfully"),
        Err(err) => {
            eprintln!("Erreur insertion Temp : {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register Temp")
        }
    }
}
