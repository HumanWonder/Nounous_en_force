//Module central des routes
pub mod register;
pub mod login;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register_user)
        // .service(register::register_temp)
        .service(login::login);
}
