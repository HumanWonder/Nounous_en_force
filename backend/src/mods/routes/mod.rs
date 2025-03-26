//Module central des routes
pub mod login;
pub mod register;
pub mod verifyemail;
pub mod profile;
pub mod admin;

use actix_web::web;
use register::{register_temp, register_user};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(verifyemail::verify_email)
        .service(register_user)
        .service(register_temp)
        .service(login::login)
        .service(login::logout)
        .service(profile::get_profile)
        .service(admin::admin_dashboard);
}
