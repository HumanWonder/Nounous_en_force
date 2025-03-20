//Routes pour inscriptions (owners et temps)
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::forms::{RegisterUser, TempRegistration};
use crate::mods::models::user::NewUser;
use crate::mods::utils::schema::{temps::dsl::*, users, users::dsl::*};
use crate::mods::utils::security::{hash_password, verify_jwt};
use crate::mods::utils::{security, send_email};
use actix_web::{http::header, post, web, HttpRequest, HttpResponse, Responder};
use chrono::Duration;
use diesel::*;
use uuid::Uuid;

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
            match send_email::send_verification_email(&data.email, &validation_token) {
                Ok(_) => Ok(HttpResponse::Ok().json("Email envoyé")),
                Err(err) => {
                    println!("Erreur d'envoi d'email: {:?}", err);
                    Err(ApiError::new("Erreur dans l'envoi de l'email", None))
                }
            }
        }
        Err(err) => {
            println!("Erreur insertion user : {:?}", err);
            Err(ApiError::new(
                "Failed to register user",
                Some("db_insert_failed".to_string()),
            ))
        }
    }
}

#[post("/register/temp")]
async fn register_temp(
    data: web::Json<TempRegistration>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> impl Responder {
    println!("Registering temp user");

    // Vérification du token JWT dans l'en-tête
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_value) = auth_header.to_str() {
            if let Some(token) = auth_value.strip_prefix("Bearer ") {
                match verify_jwt(token) {
                    Ok(user_email) => {
                        // Récupérer l'user_id de l'utilisateur à partir de son email
                        let conn = &mut pool.get().expect("Erreur connexion DB");

                        // Chercher l'ID de l'utilisateur dans la table `users`
                        match users
                            .filter(users::email.eq(&user_email))
                            .select(users::id)
                            .first::<Uuid>(conn)
                        {
                            Ok(user_user_id) => {
                                // Insérer l'enregistrement du `temp` en liant l'ID de l'utilisateur
                                let new_temp = TempRegistration {
                                    user_id: user_user_id, // Utilisation de l'user_id de l'utilisateur authentifié
                                    full_name: data.full_name.clone(),
                                    address: data.address.clone(),
                                    phone: data.phone.clone(),
                                    email: user_email.clone(),
                                    birth_date: data.birth_date.clone(),
                                    driver_license: data.driver_license,
                                    transport: data.transport.clone(),
                                    motivation: data.motivation.clone(),
                                    judicial_record: data.judicial_record.clone(),
                                };

                                // Insérer le `temp` dans la base de données
                                match insert_into(temps).values(&new_temp).execute(conn) {
                                    Ok(_) => Ok(HttpResponse::Ok().json(
                                        "Temp enregistré, en attente de validation par l'admin",
                                    )),
                                    Err(err) => {
                                        println!("Erreur insertion temp : {:?}", err);
                                        Err(ApiError::new(
                                            "Failed to register temp",
                                            Some("db_update_failed".to_string()),
                                        ))
                                    }
                                };
                            }
                            Err(_) => {
                                Err(ApiError::new(
                                    "Failed to register temp",
                                    Some("invalid_credentials".to_string()),
                                ));
                            }
                        }
                    }
                    Err(_) => Err(ApiError::new(
                        "Failed to register temp: token invalide",
                        Some("invalid_credentials".to_string()),
                    )),
                }
            }
        }
    } else {
        return Err(ApiError::new(
            "Failed to register temp",
            Some("invalid_credentials".to_string()),
        ))
    }

}
