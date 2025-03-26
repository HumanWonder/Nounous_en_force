// src/middleware.rs

use actix_web::{Error, HttpRequest, HttpResponse, Result};
use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware;
use crate::mods::utils::security::verify_jwt;
use crate::mods::models::apierror::ApiError;

// Structure du middleware
pub struct AuthMiddleware;

impl<S> Middleware<S> for AuthMiddleware {
    fn start(&self, req: &ServiceRequest) -> Result<ServiceResponse, Error> {
        // Extraction du token depuis les cookies ou les en-têtes
        if let Some(token) = req.headers().get("Authorization") {
            let token_str = token.to_str().unwrap_or_default();

            // Vérification du JWT
            match verify_jwt(&token_str) {
                Ok(email) => {
                    // Récupérer le rôle à partir des en-têtes (ou de la base de données si nécessaire)
                    if let Some(role) = req.headers().get("X-Role") {
                        let role_str = role.to_str().unwrap_or_default();
                        if role_str == "admin" {
                            return Ok(req.into_response(HttpResponse::Ok().finish())); // Permet de continuer
                        }
                    }
                    // Accès refusé si le rôle n'est pas admin
                    return Ok(req.error_response(HttpResponse::Forbidden().finish())); 
                }
                Err(_) => {
                    // Si le token est invalide
                    return Ok(req.error_response(HttpResponse::Unauthorized().finish()));
                }
            }
        }
        Err(actix_web::error::ErrorUnauthorized("Token manquant ou invalide"))
    }
}
