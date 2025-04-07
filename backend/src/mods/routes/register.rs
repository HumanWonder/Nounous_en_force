//Routes pour inscriptions (owners et temps)
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::forms::{
    RegisterUser, TempAvailabilityForm, TempConditionForm, TempDiplomaForm, TempExperienceForm,
    TempRegistration, TempRequest,
};
use crate::mods::models::temps::Temp;
use crate::mods::models::user::NewUser;
use crate::mods::utils::schema::{
    temp_availabilities::dsl::*, temp_conditions::dsl::*, temp_diplomas::dsl::*,
    temp_experiences::dsl::*, temps::dsl::*, users, users::dsl::*,
};
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
            let validation_token =
                security::generate_jwt(&data.email, None, &new_user.role, Duration::minutes(15));

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
    data: web::Json<TempRequest>,
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
        None => {
            return Err(ApiError::new(
                "Missing authentication",
                Some("invalid_credentials".to_string()),
            ))
        }
    };

    // Vérification du JWT
    let user_data = match verify_jwt(token) {
        Ok((token_email, token_role)) => (token_email, token_role),
        Err(_) => {
            return Err(ApiError::new(
                "Invalid token",
                Some("invalid_credentials".to_string()),
            ))
        }
    };

    let conn = &mut pool.get().expect("Erreur connexion DB");

    // Récupération de l'ID/role de l'utilisateur
    let db_user_id: Uuid = match users
        .filter(users::email.eq(&user_data.0))
        .select(users::id)
        .first::<Uuid>(conn)
    {
        Ok(db_id_data) => db_id_data,
        Err(_) => {
            return Err(ApiError::new(
                "User not found",
                Some("invalid_credentials".to_string()),
            ))
        }
    };

    // Vérifier si l'utilisateur est bien "pending"
    if user_data.1 != "pending" {
        return Err(ApiError::new(
            "User already registered with a different role",
            Some("db_update_failed".to_string()),
        ));
    }

    // Création de l'enregistrement pour `temps` (table profile intervenant.e)
    let new_temp = TempRegistration {
        user_id: db_user_id,
        full_name: data.temp_info.full_name.clone(),
        address: data.temp_info.address.clone(),
        phone: data.temp_info.phone.clone(),
        birth_date: data.temp_info.birth_date.clone(),
        driver_license: data.temp_info.driver_license,
        transport: data.temp_info.transport.clone(),
        motivation: data.temp_info.motivation.clone(),
        judicial_record: data.temp_info.judicial_record.clone(),
    };

    let mut availability_inserts = Vec::new(); //Vecteur pour insérer plusieurs données: table = Vec
    let mut condition_inserts = Vec::new();
    let mut document_inserts = Vec::new();
    let mut experience_inserts = Vec::new();

    for availability_data in &data.availabilities {
        let new_availability = TempAvailabilityForm {
            temp_id: db_user_id,
            available_periods: availability_data.available_periods.clone(),
            work_hours: availability_data.work_hours.clone(),
            preferred_locations: availability_data.preferred_locations.clone(),
            max_travel_time: availability_data.max_travel_time.clone(),
        };
        availability_inserts.push(new_availability);
    }

    // Insertion des horaires de travail
    for work_data in &data.work_hours {
        let new_condition = TempConditionForm {
            temp_id: db_user_id,
            hourly_rate: work_data.hourly_rate.clone(),
            contract_types: work_data.contract_types.clone(),
            self_employment: work_data.self_employment,
        };
        condition_inserts.push(new_condition);
    }

    // Insertion des diplômes
    for diploma_data in &data.documents {
        let new_diploma = TempDiplomaForm {
            temp_id: db_user_id,
            diploma_name: diploma_data.diploma_name.clone(),
            other_certifications: diploma_data.other_certifications.clone(),
            year_obtained: diploma_data.year_obtained,
            institution: diploma_data.institution.clone(),
        };
        document_inserts.push(new_diploma);
    }

    // Insertion des expériences
    for experience_data in &data.experiences {
        let new_experience = TempExperienceForm {
            temp_id: db_user_id,
            total_experience: experience_data.total_experience.clone(),
            previous_jobs: experience_data.previous_jobs.clone(),
            structure_types: experience_data.structure_types.clone(),
            tasks: experience_data.tasks.clone(),
        };
        experience_inserts.push(new_experience);
    }

    // Transaction dans la base de données (multiples tables)
    match conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Insertion du profil dans `temps` et récupération de l'id
        let inserted_temp: Temp = insert_into(temps).values(&new_temp).get_result(conn)?;

        let inserted_temp_id = inserted_temp.id;

        // Insertion des disponibilités
        let availability_inserts: Vec<_> = data
            .availabilities
            .iter()
            .map(|availability_data| TempAvailabilityForm {
                temp_id: inserted_temp_id,
                available_periods: availability_data.available_periods.clone(),
                work_hours: availability_data.work_hours.clone(),
                preferred_locations: availability_data.preferred_locations.clone(),
                max_travel_time: availability_data.max_travel_time.clone(),
            })
            .collect();

        insert_into(temp_availabilities)
            .values(&availability_inserts)
            .execute(conn)?;

        // Insertion des conditions de travail
        let condition_inserts: Vec<_> = data
            .work_hours
            .iter()
            .map(|work_data| TempConditionForm {
                temp_id: inserted_temp_id,
                hourly_rate: work_data.hourly_rate.clone(),
                contract_types: work_data.contract_types.clone(),
                self_employment: work_data.self_employment,
            })
            .collect();

        insert_into(temp_conditions)
            .values(&condition_inserts)
            .execute(conn)?;

        // Insertion des diplômes
        let document_inserts: Vec<_> = data
            .documents
            .iter()
            .map(|diploma_data| TempDiplomaForm {
                temp_id: inserted_temp_id,
                diploma_name: diploma_data.diploma_name.clone(),
                other_certifications: diploma_data.other_certifications.clone(),
                year_obtained: diploma_data.year_obtained,
                institution: diploma_data.institution.clone(),
            })
            .collect();

        insert_into(temp_diplomas)
            .values(&document_inserts)
            .execute(conn)?;

        // Insertion des expériences
        let experience_inserts: Vec<_> = data
            .experiences
            .iter()
            .map(|experience_data| TempExperienceForm {
                temp_id: inserted_temp_id,
                total_experience: experience_data.total_experience.clone(),
                previous_jobs: experience_data.previous_jobs.clone(),
                structure_types: experience_data.structure_types.clone(),
                tasks: experience_data.tasks.clone(),
            })
            .collect();

        insert_into(temp_experiences)
            .values(&experience_inserts)
            .execute(conn)?;

        Ok(inserted_temp_id)
    }) {
        Ok(_) => {
            // Mise à jour du rôle en "temp"
            match diesel::update(users.filter(users::id.eq(db_user_id)))
                .set(users::role.eq("temp"))
                .execute(conn)
            {
                Ok(_) => println!("User role updated to 'temp'"),
                Err(err) => {
                    println!("Erreur mise à jour rôle : {:?}", err);
                    return Err(ApiError::new(
                        "Failed to update user role",
                        Some("db_update_failed".to_string()),
                    ));
                }
            };
            Ok(HttpResponse::Ok().json("Temp enregistré, en attente de validation par l'admin"))
        }
        Err(err) => {
            println!("Erreur insertion temp : {:?}", err);
            Err(ApiError::new(
                "Failed to register temp",
                Some("db_insert_failed".to_string()),
            ))
        }
    }
}
