mod utils;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, test, http::StatusCode};
use bcrypt::{hash, DEFAULT_COST};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection, RunQueryDsl,
};
use dotenv::dotenv;
use std::env;
use utils::auth;
use utils::schema::users::dsl::*;

//Garde des connexions à la base de données ouvertes, individuelles et accessibles.
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::test]
async fn test_register() {
    use serde_json::json;
    let database_url = "postgres://myuser:mvtmjsun@localhost/nnef";  // Ton URL DB
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(register), // Route à tester
    )
    .await;

    let user_data = json!({
        "email": "other_test@testing.test",
        "password": "abcde_bad_password",
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&user_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello zere! Velcome")
}

#[post("/register")]
async fn register(
    data: web::Json<auth::RegisterRequest>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Error while connecting to DB");
    let other_hashed_password = hash(&data.password, DEFAULT_COST)
    .expect("Failed to hash password")
    .to_string();

    let new_user = auth::NewUser {
        email: data.email.clone(),
        hashed_password: other_hashed_password,
    };
    // Insérer en DB
    match diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(err) => {
            eprintln!("Error inserting user: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Récupère l'URL de la base de données
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    // Crée un gestionnaire de connexion
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Initialise un pool de connexions (sorte de hall d'entrée)
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Échec de la création du pool de connexions");

    //Lance le serveur
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Ajoute le pool au serveur
            .service(hello)
            .service(register)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
