use actix_web::{cookie::{time::Duration, Cookie}, HttpRequest};
//Gestion hash et JWT
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use dotenv::dotenv;

use crate::mods::models::apierror::ApiError;

//--------------------A SUPPRIMER !!!------------------- faire env key
const SECRET_KEY: &[u8] = b"supersecretkey";
//--------------------A SUPPRIMER !!!-------------------

// Structure pour les claims du JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: Option<String>,
    token_id: String, // Identifiant unique pour chaque token
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Erreur de hashage du mot de passe")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

pub fn create_auth_cookie(token: String) -> Cookie<'static> {
    //Même temps d'expiration que le token de session active
    let expiration_time = Duration::hours(2);
    Cookie::build("auth_token", token)
        .http_only(true)    // Empêche l'accès au token via JS (protection XSS)
        // Doit être sécurisé en production (HTTPS)
        // .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)//Contre les attaques CSRF (utilisation de la session active d'un utilisateur pour qu'il fasse une requête malicieuse souvent par l'intermédiaire d'un lien)
        .max_age(expiration_time)
        .finish()
}

pub fn generate_jwt(email: &str, user_id: Option<Uuid>, duration: chrono::Duration) -> String {
    // Calculer l'expiration du token (par exemple ici dans 15min)
    let expiration = (Utc::now() + duration).timestamp() as usize;

    let claims = Claims {
        sub: email.to_string(),
        //Renvoie None si user_id n'existe pas
        user_id: user_id.map(|id| id.to_string()),
        token_id: Uuid::new_v4().to_string(),
        exp: expiration,
    };

    encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
    .expect("Erreur génération token")
}

pub fn verify_jwt(token: &str) -> Result<String, String> {
    // let SECRET_KEY = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => Ok(data.claims.sub), // Retourne l'email
        Err(_) => Err("Token invalide".to_string()),
    }
}

//Fonction qui check si le cookie contient toujours un token valide. Permet d'authentifier le user et permettre sa requête
pub fn extract_token_from_cookie(req: &HttpRequest) -> Result<String, ApiError> {
    if let Some(cookie) = req.cookie("auth_token") {
        let token = cookie.value().to_string();
        match verify_jwt(&token) {
            Ok(email) => Ok(email), // Retourne l'email si le token est valide
            Err(_) => Err(ApiError::new("Token invalide", Some("invalid_credentials".to_string()))),
        }
    } else {
        Err(ApiError::new("Token manquant", Some("invalid_credentials".to_string())))
    }
}

//A voir si utile au déploiement
pub fn get_front_conn() -> String {
    dotenv().ok();
    env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    // Valeur par défaut
}
