//Routes pour inscriptions (owners et temps)
use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;
use crate::mods::models::forms::{
    RegisterUser, TempAvailabilityForm, TempConditionForm, TempDiplomaForm, TempDocumentForm,
    TempExperienceForm, TempRegistration, TempRequest, TempSkillForm,
};
use crate::mods::models::temps::Temp;
use crate::mods::models::user::NewUser;
use crate::mods::utils::schema::{
    temp_availabilities::dsl::*, temp_conditions::dsl::*, temp_diplomas::dsl::*,
    temp_documents::dsl::*, temp_experiences::dsl::*, temp_skills::dsl::*, temps::dsl::*, users,
    users::dsl::*,
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
        user_id: Some(db_user_id),
        first_name: data.temp_info.first_name.clone(),
        last_name: data.temp_info.last_name.clone(),
        address: data.temp_info.address.clone(),
        phone: data.temp_info.phone.clone(),
        email: user_data.0.clone(),
        birth_date: data.temp_info.birth_date.clone(),
        has_driver_license: data.temp_info.has_driver_license,
        transport_mode: data.temp_info.transport_mode.clone(),
    };

    // let mut availability_inserts = Vec::new(); //Vecteur pour insérer plusieurs données: table = Vec
    // let mut condition_inserts = Vec::new();
    // let mut diploma_inserts = Vec::new();
    // let mut experience_inserts = Vec::new();
    // let mut skill_inserts = Vec::new();
    // let mut documents_inserts = Vec::new();

    // for availability_data in &data.availabilities {
    //     let new_availability = TempAvailabilityForm {
    //         temp_id: Some(db_user_id),
    //         availability_periods: availability_data.availability_periods.clone(),
    //         time_slots: availability_data.time_slots.clone(),
    //         geographic_zones: availability_data.geographic_zones.clone(),
    //         max_travel_time: availability_data.max_travel_time.clone(),
    //     };
    //     availability_inserts.push(new_availability);
    // }

    // // Insertion des horaires de travail
    // for work_data in &data.conditions {
    //     let new_condition = TempConditionForm {
    //         temp_id: Some(db_user_id),
    //         hourly_rate: work_data.hourly_rate.clone(),
    //         contract_types: work_data.contract_types.clone(),
    //         auto_entrepreneur: work_data.auto_entrepreneur,
    //     };
    //     condition_inserts.push(new_condition);
    // }

    // // Insertion des diplômes
    // for diploma_data in &data.diplomas {
    //     let new_diploma = TempDiplomaForm {
    //         temp_id: Some(db_user_id),
    //         main_diploma: diploma_data.main_diploma.clone(),
    //         other_certifications: diploma_data.other_certifications.clone(),
    //         graduation_year: diploma_data.graduation_year,
    //         school: diploma_data.school.clone(),
    //     };
    //     diploma_inserts.push(new_diploma);
    // }

    // // Insertion des expériences
    // for experience_data in &data.experiences {
    //     let new_experience = TempExperienceForm {
    //         temp_id: Some(db_user_id),
    //         total_experience: experience_data.total_experience.clone(),
    //         previous_positions: experience_data.previous_positions.clone(),
    //         structure_types: experience_data.structure_types.clone(),
    //         tasks: experience_data.tasks.clone(),
    //     };
    //     experience_inserts.push(new_experience);
    // }

    // //Insertion des skills
    // for skill_data in &data.skills {
    //     let new_skill = TempSkillForm {
    //         temp_id: Some(db_user_id),
    //         languages: skill_data.languages.clone(),
    //         pedagogies: skill_data.pedagogies.clone(),
    //         special_skills: skill_data.special_skills.clone(),
    //         special_needs_handling: skill_data.special_needs_handling.clone(),
    //     };
    //     skill_inserts.push(new_skill);
    // }

    // //Insertion des documents
    // for doc_data in &data.documents {
    //     let new_doc = TempDocumentForm {
    //         temp_id: Some(db_user_id),
    //         motivation_letter: doc_data.motivation_letter.clone(),
    //         professional_references: doc_data.professional_references.clone(),
    //         diplomas: doc_data.diplomas.clone(),
    //         criminal_record: doc_data.criminal_record.clone(),
    //         required_documents: doc_data.required_documents.clone(),
    //     };
    //     documents_inserts.push(new_doc);
    // }

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
                temp_id: Some(inserted_temp_id),
                availability_periods: availability_data.availability_periods.clone(),
                time_slots: availability_data.time_slots.clone(),
                geographic_zones: availability_data.geographic_zones.clone(),
                max_travel_time: availability_data.max_travel_time.clone(),
            })
            .collect();

        insert_into(temp_availabilities)
            .values(&availability_inserts)
            .execute(conn)?;

        // Insertion des conditions de travail
        let condition_inserts: Vec<_> = data
            .conditions
            .iter()
            .map(|work_data| TempConditionForm {
                temp_id: Some(inserted_temp_id),
                hourly_rate: work_data.hourly_rate.clone(),
                contract_types: work_data.contract_types.clone(),
                auto_entrepreneur: work_data.auto_entrepreneur,
            })
            .collect();

        insert_into(temp_conditions)
            .values(&condition_inserts)
            .execute(conn)?;

        // Insertion des diplômes
        let diploma_inserts: Vec<_> = data
            .diplomas
            .iter()
            .map(|diploma_data| TempDiplomaForm {
                temp_id: Some(inserted_temp_id),
                main_diploma: diploma_data.main_diploma.clone(),
                other_certifications: diploma_data.other_certifications.clone(),
                graduation_year: diploma_data.graduation_year,
                school: diploma_data.school.clone(),
            })
            .collect();

        insert_into(temp_diplomas)
            .values(&diploma_inserts)
            .execute(conn)?;

        // Insertion des expériences
        let experience_inserts: Vec<_> = data
            .experiences
            .iter()
            .map(|experience_data| TempExperienceForm {
                temp_id: Some(inserted_temp_id),
                total_experience: experience_data.total_experience.clone(),
                previous_positions: experience_data.previous_positions.clone(),
                structure_types: experience_data.structure_types.clone(),
                tasks: experience_data.tasks.clone(),
            })
            .collect();

        insert_into(temp_experiences)
            .values(&experience_inserts)
            .execute(conn)?;

        // Insertion des compétences
        let skill_inserts: Vec<_> = data
            .skills
            .iter()
            .map(|skill_data| TempSkillForm {
                temp_id: Some(inserted_temp_id),
                languages: skill_data.languages.clone(),
                pedagogies: skill_data.pedagogies.clone(),
                special_skills: skill_data.special_skills.clone(),
                special_needs_handling: skill_data.special_needs_handling.clone(),
            })
            .collect();

        insert_into(temp_skills)
            .values(&skill_inserts)
            .execute(conn)?;

        //Insertion des documents
        let documents_inserts: Vec<_> = data
            .documents
            .iter()
            .map(|doc_data| TempDocumentForm {
                temp_id: Some(inserted_temp_id),
                motivation_letter: doc_data.motivation_letter.clone(),
                professional_references: doc_data.professional_references.clone(),
                diplomas: doc_data.diplomas.clone(),
                criminal_record: doc_data.criminal_record.clone(),
                required_documents: doc_data.required_documents.clone(),
            })
            .collect();

        insert_into(temp_documents)
            .values(&documents_inserts)
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
