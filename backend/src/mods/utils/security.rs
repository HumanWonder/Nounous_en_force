//Gestion hash et JWT
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;
use uuid::Uuid;
use chrono::{Duration, Utc};

//faire env key
const SECRET_KEY: &[u8] = b"supersecretkey";

#[derive(Serialize)]
struct Claims {
    sub: String,
    token_id: String,    // Identifiant unique pour chaque token
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Erreur de hashage du mot de passe")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

pub fn generate_jwt(email: &str) -> String {
    // Calculer l'expiration du token (par exemple dans 1 heure)
    let expiration = (Utc::now() + Duration::hours(1)).timestamp() as usize;

    let claims = Claims {
        sub: email.to_string(),
        token_id: Uuid::new_v4().to_string(),
        exp: expiration, 
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
        .expect("Erreur génération token")
}
