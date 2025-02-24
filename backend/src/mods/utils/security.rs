//Gestion hash et JWT
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;

const SECRET_KEY: &[u8] = b"supersecretkey";

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Erreur de hashage du mot de passe")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}

pub fn generate_jwt(email: &str) -> String {
    let claims = Claims {
        sub: email.to_string(),
        exp: 10000000000, // Expiration du token (timestamp)
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
        .expect("Erreur génération token")
}
