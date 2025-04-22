// Gestion pool de connexions
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;

// Définition du type du pool de connexions
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Échec de la création du pool de connexions")
}
