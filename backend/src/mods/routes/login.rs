// Routes pour connexion et JWT
use actix_web::{post, web, HttpResponse, Responder};
use diesel::*;
use dsl::not;
use uuid::Uuid;
use crate::db::DbPool;
use crate::mods::models::forms::{LoginResponse, LoginUser};
use crate::mods::utils::schema::users::dsl::*;
use crate::mods::utils::security::{verify_password, generate_jwt};

#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginUser>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Erreur connexion DB");
    println!("Logging in user");

    let user = match users 
        .filter(email.eq(&credentials.email))
        .select((id, hashed_password, email, is_validated))
        .first::<(Uuid, String, String, bool)>(conn)
        .optional().unwrap() {
            Some(user) => Some(user),
            None => None,
        };

        println!("Checking credentials : {:?}", user);
    match user {
        Some(user) => {
            if !bool::from(user.3) {
                return HttpResponse::Unauthorized().body("Veuillez valider votre email");
            };
            if verify_password(&credentials.password, &user.1) {
                let token = generate_jwt(&user.2);
                let response = LoginResponse {
                    id: user.0,
                    token,
                };
                HttpResponse::Ok().json(response)
            } else {
                HttpResponse::Unauthorized().body("Mot de passe incorrect")
            }
        }
        None => HttpResponse::Unauthorized().body("Utilisateur non trouvé"),
    }
}
