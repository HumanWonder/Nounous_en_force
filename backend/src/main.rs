mod db;
mod mods;
use actix_cors::Cors;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use db::init_pool;
use mods::{routes::init_routes, utils::security::get_front_conn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = init_pool();

    HttpServer::new(move || {
        let cors = Cors::default()  //Autorise les requêtes originant du front
            .allowed_origin(&get_front_conn())//charge l'url
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
