//Module central des routes
pub mod admin;
pub mod login;
pub mod profile;
pub mod register;
pub mod verifyemail;

use actix_web::web;
use register::{register_owner::register_owner, register_temp::register_temp, register_user::register_user};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(verifyemail::verify_email)
        .service(register_user)
        .service(register_temp)
        .service(register_owner)
        .service(login::login)
        .service(login::logout)
        .service(profile::get_profile)
        .service(admin::admin_dashboard)
        .service(admin::validate_user);
}
