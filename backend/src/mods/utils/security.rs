use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    HttpRequest,
};
//Gestion hash et JWT
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::mods::models::apierror::ApiError;

//--------------------A SUPPRIMER !!!------------------- faire env key
const SECRET_KEY: &[u8] = b"supersecretkey";
//--------------------A SUPPRIMER !!!-------------------

// Structure pour les claims du JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    //sub pour subject. Le renvoi principal du token
    sub: String,
    user_id: Option<String>,
    token_id: String, // Identifiant unique pour chaque token
    exp: usize,
    role: String,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Erreur de hashage du mot de passe")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

pub fn generate_jwt(
    email: &str,
    user_id: Option<Uuid>,
    role: &str,
    duration: chrono::Duration,
) -> String {
    // Calculer l'expiration du token (par exemple ici dans 15min)
    let expiration = (Utc::now() + duration).timestamp() as usize;

    let claims = Claims {
        sub: email.to_string(),
        //Renvoie None si user_id n'existe pas
        user_id: user_id.map(|id| id.to_string()),
        token_id: Uuid::new_v4().to_string(),
        exp: expiration,
        role: role.to_string(),
    };

    encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
    .expect("Erreur génération token")
}

pub fn verify_jwt(token: &str) -> Result<(String, String), String> {
    // let SECRET_KEY = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => Ok((data.claims.sub, data.claims.role)), // Retourne l'email et le rôle du user
        Err(_) => Err("Token invalide".to_string()),
    }
}

//Création d'un cookie pour session de conenxion d'un utilisateur
pub fn create_auth_cookie(token: Option<String>) -> Cookie<'static> {
    //Même temps d'expiration que le token de session active
    let expiration_time = Duration::hours(2);

    let cookie = match token {
        Some(token) => {
            Cookie::build("auth_token", token)
                .path("/") //Permet au front d'accéder au cookie
                .http_only(true) // Empêche l'accès au token via JS (protection XSS)
                // Doit être sécurisé en production (HTTPS)
                // .secure(true)  //Passer à true en prod
                // .same_site(SameSite::None)//Contre les attaques CSRF (utilisation de la session active d'un utilisateur pour qu'il fasse une requête malicieuse souvent par l'intermédiaire d'un lien)
                .max_age(expiration_time)
                .finish()
        }

        None => {
            // Cookie vide et expiré pour la déconnexion
            Cookie::build("auth_token", "")
                .http_only(true)
                .same_site(SameSite::Strict)
                .max_age(Duration::seconds(0)) // Expire immédiatement
                .finish()
        }
    };
    cookie
}

//Fonction qui check si le cookie contient toujours un token valide. Permet d'authentifier le user et permettre sa requête
pub fn extract_token_from_cookie(req: &HttpRequest) -> Result<(String, String), ApiError> {
    if let Some(cookie) = req.cookie("auth_token") {
        let token = cookie.value().to_string();
        match verify_jwt(&token) {
            Ok((email, role)) => Ok((email, role)), // Retourne l'email et le rôle si le token est valide
            Err(_) => Err(ApiError::new(
                "Token invalide",
                Some("invalid_credentials".to_string()),
            )),
        }
    } else {
        Err(ApiError::new(
            "Token manquant",
            Some("invalid_credentials".to_string()),
        ))
    }
}

//A voir si utile au déploiement
pub fn get_front_conn() -> String {
    dotenv().ok();
    env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    // Valeur par défaut
}
