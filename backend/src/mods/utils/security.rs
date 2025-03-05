//Gestion hash et JWT
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use dotenv::dotenv;

//--------------------A SUPPRIMER !!!------------------- faire env key
const SECRET_KEY: &[u8] = b"supersecretkey";
//--------------------A SUPPRIMER !!!-------------------

// Structure pour les claims du JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    token_id: String, // Identifiant unique pour chaque token
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Erreur de hashage du mot de passe")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

pub fn generate_jwt(email: &str) -> String {
    // Calculer l'expiration du token (par exemple ici dans 15min)
    let expiration = (Utc::now() + Duration::minutes(15)).timestamp() as usize;

    let claims = Claims {
        sub: email.to_string(),
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

pub fn get_front_conn() -> String {
    dotenv().ok();
    env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    // Valeur par défaut
}
