// Routes pour connexion et JWT
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::mods::models::user::User;
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::{verify_password, generate_jwt};

#[post("/login")]
async fn login(
    credentials: web::Json<User>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Erreur connexion DB");

    let user = users
        .filter(email.eq(&credentials.email))
        .first::<User>(conn)
        .optional()
        .expect("Erreur requête utilisateur");

    match user {
        Some(user) => {
            if verify_password(&credentials.hashed_password, &user.hashed_password) {
                let token = generate_jwt(&user.email);
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().body("Mot de passe incorrect")
            }
        }
        None => HttpResponse::Unauthorized().body("Utilisateur non trouvé"),
    }
}
