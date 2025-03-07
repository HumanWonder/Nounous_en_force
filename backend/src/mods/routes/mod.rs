//Module central des routes
pub mod login;
pub mod register;
pub mod verifyemail;
pub mod profile;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(verifyemail::verify_email)
        .service(register::register_user)
        .service(login::login)
        .service(login::logout)
        .service(profile::get_profile);
}
