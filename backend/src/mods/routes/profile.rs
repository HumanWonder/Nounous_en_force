use crate::db::DbPool;
use crate::mods::models::apierror::ApiError;

//import des structures pour le profil
use crate::mods::models::{
    nurseries::OwnerProfile,
    temps::{
        Temp, TempAvailabilitie, TempCondition, TempDiploma, TempDocument, TempExperience,
        TempProfile, TempSkill,
    },
    user::{FullProfileData, User},
};

//définition spécifiques de user_id pour ne pas confondre
use crate::mods::utils::schema::{
    nurseries::dsl::{nurseries, nursery_id as nursery_owner_id},
    temp_availabilities::dsl::{temp_availabilities, temp_id as dispo_temp_id},
    temp_conditions::dsl::{temp_conditions, temp_id as cond_temp_id},
    temp_diplomas::dsl::{temp_diplomas, temp_id as diplo_temp_id},
    temp_documents::dsl::{temp_documents, temp_id as doc_temp_id},
    temp_experiences::dsl::{temp_experiences, temp_id as exp_temp_id},
    temp_skills::dsl::{temp_id as skill_temp_id, temp_skills},
    temps::dsl::{temps, user_id as temp_user_id},
    users::dsl::{email, users},
};

use crate::mods::utils::security;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/profile")]
pub async fn get_profile(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    println!("Getting profile info....");
    match security::extract_token_from_cookie(&req) {
        Ok((mail, _)) => {
            // Connexion à la base de données
            let conn = &mut pool
                .get()
                .expect("Erreur de connexion à la base de données");

            // Chercher les informations de l'utilisateur dans la base de données
            let user_info: User = match users
                .filter(email.eq(mail)) // Filtrer par email
                .select(User::as_select())
                .first::<User>(conn) // Récupérer les données
                .optional()
                .map_err(|e| {
                    ApiError::new(
                        "Erreur lors de la recherche utilisateur",
                        Some(e.to_string()),
                    )
                })? {
                Some(info) => info,
                None => {
                    return Err(ApiError::new(
                        "Utilisateur non trouvé",
                        Some("user_not_found".to_string()),
                    ))
                }
            };
            // Utilisation d'un match pour gérer les différents rôles
            match user_info.role.as_str() {
                "temp" => {
                    println!("Constructing TempProfile for user ID: {:?}", user_info.id);
                    let temp_info = temps
                        .filter(temp_user_id.eq(user_info.id))
                        .first::<Temp>(conn)
                        .map_err(|e| {
                            ApiError::new("Erreur profil intérimaire", Some(e.to_string()))
                        })?;

                    let availabilities_list = temp_availabilities
                        .filter(dispo_temp_id.eq(temp_info.id))
                        .load::<TempAvailabilitie>(conn)
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors du chargement des disponibilités",
                                Some(e.to_string()),
                            )
                        })?;

                    let conditions_list = temp_conditions
                        .filter(cond_temp_id.eq(temp_info.id))
                        .load::<TempCondition>(conn)
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors du chargement des conditions de travail",
                                Some(e.to_string()),
                            )
                        })?;

                    let diplomas_list = temp_diplomas
                        .filter(diplo_temp_id.eq(temp_info.id))
                        .load::<TempDiploma>(conn)
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors du chargement des certifications",
                                Some(e.to_string()),
                            )
                        })?;

                    let experiences_list = temp_experiences
                        .filter(exp_temp_id.eq(temp_info.id))
                        .load::<TempExperience>(conn)
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors du chargement des experiences",
                                Some(e.to_string()),
                            )
                        })?;

                    let skills_list = temp_skills
                        .filter(skill_temp_id.eq(temp_info.id))
                        .load::<TempSkill>(conn)
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors du chargement des compétences",
                                Some(e.to_string()),
                            )
                        })?;

                    let docs_list = temp_documents
                        .filter(doc_temp_id.eq(temp_info.id))
                        .load::<TempDocument>(conn)
                        .map_err(|e| {
                            ApiError::new(
                                "Erreur lors du chargement des documents",
                                Some(e.to_string()),
                            )
                        })?;

                    let profile = FullProfileData::Temp {
                        user: user_info,
                        temp: TempProfile {
                            temp: temp_info,
                            availabilities: availabilities_list,
                            conditions: conditions_list,
                            diplomas: diplomas_list,
                            experiences: experiences_list,
                            skills: skills_list,
                            documents: docs_list,
                        },
                    };
                    println!(
                        "profile JSON:\n{}",
                        serde_json::to_string_pretty(&profile).unwrap()
                    );

                    // println!("full profile data temp -> {:?}", availabilities_list[0]);

                    Ok(HttpResponse::Ok().json(profile))
                }
                "owner" => {
                    let owner_info = nurseries
                        .filter(nursery_owner_id.eq(user_info.id))
                        .first::<OwnerProfile>(conn)
                        .map_err(|e| {
                            ApiError::new("Erreur profil responsable", Some(e.to_string()))
                        })?;

                    let profile = FullProfileData::Owner {
                        user: user_info,
                        owner_info: OwnerProfile {
                            nursery: (),
                            responsible: (),
                            description: (),
                            needs: (),
                        },
                    };

                    Ok(HttpResponse::Ok().json(profile))
                }
                _ => {
                    let profile = FullProfileData::Basic { user: user_info };
                    Ok(HttpResponse::Ok().json(profile))
                }
            }
        }
        Err(err) => Err(err), // Renvoie une 401 si le token est invalide
    }
}
