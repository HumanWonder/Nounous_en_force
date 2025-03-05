use actix_web::{HttpResponse, ResponseError};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub status: String,        // Exemple : "error"
    pub message: String,       // Le message d'erreur
    pub error_code: Option<String>, // Code d'erreur spécifique (si besoin)
}

impl ApiError {
    // Constructeur pour créer un ApiError générique
    pub fn new(message: &str, error_code: Option<String>) -> Self {
        ApiError {
            status: "error".to_string(),
            message: message.to_string(),
            error_code,
        }
    }

    // Constructeur pour un message d'erreur sans code d'erreur
    pub fn without_code(message: &str) -> Self {
        ApiError::new(message, None)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_code.as_deref() {
            Some("invalid_credentials") => actix_web::http::StatusCode::UNAUTHORIZED,
            Some("user_not_found") => actix_web::http::StatusCode::NOT_FOUND,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(self)
    }
}
