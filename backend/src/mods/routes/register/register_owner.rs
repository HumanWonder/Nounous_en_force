use actix_web::{http::header, post, web, HttpRequest, HttpResponse};
use diesel::{insert_into, prelude::*};
use uuid::Uuid;

use crate::db::DbPool;
use crate::mods::models::forms::OwnerRequest;
use crate::mods::models::{
    apierror::ApiError,
    forms::{NewNurseryForm, NurseryDescriptionForm, NurseryResponsibleForm, ReplacementNeedForm},
    nurseries::Nursery,
};
use crate::mods::utils::{
    schema::{
        nurseries::dsl::*, nursery_description::dsl::*, nursery_responsibles::dsl::*,
        replacement_needs::dsl::*, users, users::dsl::*,
    },
    security::verify_jwt,
};

#[post("/register/owner")]
pub async fn register_owner(
    data: web::Json<OwnerRequest>,
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

    // Transaction dans la base de données (multiples tables)
    match conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Insertion de la crèche (nursery)
        let new_nursery = NewNurseryForm {
            user_id: Some(db_user_id),
            name: data.nursery.name.clone(),
            address: data.nursery.address.clone(),
            phone: data.nursery.phone.clone(),
            email: data.nursery.email.clone(),
            website: data.nursery.website.clone(),
            structure_type: data.nursery.structure_type.clone(),
        };

        // Insertion du profil dans `temps` et récupération de l'id
        let inserted_nursery: Nursery = insert_into(nurseries)
            .values(&new_nursery)
            .get_result(conn)?;

        let inserted_nursery_id = inserted_nursery.id;

        // Insertion de la description si fournie
        if let Some(description) = &data.description {
            let desc_form = NurseryDescriptionForm {
                nursery_id: Some(inserted_nursery_id),
                pedagogy: description.pedagogy.clone(),
                specificities: description.specificities.clone(),
                philosophy: description.philosophy.clone(),
            };

            insert_into(nursery_description)
                .values(&desc_form)
                .execute(conn)?;
        }

        // Insertion du/de la responsable si fourni(e)
        if let Some(responsible) = &data.responsible {
            let resp_form = NurseryResponsibleForm {
                nursery_id: Some(inserted_nursery_id),
                first_name: responsible.first_name.clone(),
                last_name: responsible.last_name.clone(),
                role: responsible.role.clone(),
                direct_phone: responsible.direct_phone.clone(),
                direct_email: responsible.direct_email.clone(),
            };

            insert_into(nursery_responsibles)
                .values(&resp_form)
                .execute(conn)?;
        }

        // Insertion des besoins de remplacement
        let needs_inserts: Vec<_> = data
            .needs
            .iter()
            .map(|need| ReplacementNeedForm {
                nursery_id: Some(inserted_nursery_id),
                searched_position: need.searched_position.clone(),
                replacement_reason: need.replacement_reason.clone(),
                estimated_duration: need.estimated_duration.clone(),
                available_periods: need.available_periods.clone(),
                hours_per_week: need.hours_per_week.clone(),
                main_tasks: need.main_tasks.clone(),
                required_skills: need.required_skills.clone(),
                suggested_rate: need.suggested_rate.clone(),
            })
            .collect();

        insert_into(replacement_needs)
            .values(&needs_inserts)
            .execute(conn)?;

        Ok(inserted_nursery_id)
    }) {
        Ok(_) => {
            // Mise à jour du rôle en "temp"
            match diesel::update(users.filter(users::id.eq(db_user_id)))
                .set(users::role.eq("owner"))
                .execute(conn)
            {
                Ok(_) => println!("User role updated to 'owner'"),
                Err(err) => {
                    println!("Erreur mise à jour rôle : {:?}", err);
                    return Err(ApiError::new(
                        "Failed to update user role",
                        Some("db_update_failed".to_string()),
                    ));
                }
            };
            Ok(HttpResponse::Ok()
                .json("Responsable enregistré, en attente de validation par l'admin"))
        }
        Err(err) => {
            println!("Erreur insertion owner : {:?}", err);
            Err(ApiError::new(
                "Failed to register owner",
                Some("db_insert_failed".to_string()),
            ))
        }
    }
}
