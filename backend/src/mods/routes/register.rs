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
            let validation_token = security::generate_jwt(&data.email, None, &new_user.role,Duration::minutes(15));

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
) -> Result<HttpResponse, ApiError> {
    println!("Registering temp user");

    // Vérification du token JWT dans l'en-tête ou via cookie (préparation transition)
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "));
        // .or_else(|| req.cookie("auth_token").map(|c| c.value())); // Extraction du cookie
    println!("Token : {:?}", token);

    let token = match token {
        Some(t) => t,
        None => return Err(ApiError::new("Missing authentication", Some("invalid_credentials".to_string()))),
    };

    // Vérification du JWT
    let user_data = match verify_jwt(token) {
        Ok((token_email, token_role)) => (token_email, token_role),
        Err(_) => return Err(ApiError::new("Invalid token", Some("invalid_credentials".to_string()))),
    };

    let conn = &mut pool.get().expect("Erreur connexion DB");

    // Récupération de l'ID/role de l'utilisateur
    let db_user_id: Uuid = match users.filter(users::email.eq(&user_data.0)).select(users::id).first::<Uuid>(conn) {
        Ok(db_id_data) => db_id_data,
        Err(_) => return Err(ApiError::new("User not found", Some("invalid_credentials".to_string()))),
    };

    // Vérifier si l'utilisateur est bien "pending"
    if user_data.1 != "pending" {
        return Err(ApiError::new(
            "User already registered with a different role",
            Some("db_update_failed".to_string()),
        ));
    }

    // Mise à jour du rôle en "temp"
    match diesel::update(users.filter(users::id.eq(db_user_id)))
        .set(users::role.eq("temp"))
        .execute(conn)
    {
        Ok(_) => println!("User role updated to 'temp'"),
        Err(err) => {
            println!("Erreur mise à jour rôle : {:?}", err);
            return Err(ApiError::new("Failed to update user role", Some("db_update_failed".to_string())));
        }
    };

    // Création de l'enregistrement pour `temps` (table profile intervenant.e)
    let new_temp = TempRegistration {
        user_id: db_user_id,
        full_name: data.full_name.clone(),
        address: data.address.clone(),
        phone: data.phone.clone(),
        birth_date: data.birth_date.clone(),
        driver_license: data.driver_license,
        transport: data.transport.clone(),
        motivation: data.motivation.clone(),
        judicial_record: data.judicial_record.clone(),
    };

    // Insertion dans la base de données
    match insert_into(temps).values(&new_temp).execute(conn) {
        Ok(_) => Ok(HttpResponse::Ok().json("Temp enregistré, en attente de validation par l'admin")),
        Err(err) => {
            println!("Erreur insertion temp : {:?}", err);
            Err(ApiError::new("Failed to register temp", Some("db_insert_failed".to_string())))
        }
    }
}

