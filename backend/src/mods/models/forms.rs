// Struct des formulaires d'inscription
use serde::Deserialize;
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}


// #[derive(Deserialize)]
// pub struct RegisterOwner {
//     pub email: String,
//     pub password: String,
//     pub last_name: String,
//     pub first_name: String,
//     pub job_position: Option<String>,
//     pub tel_number: Option<String>,
//     pub address: Option<String>,
// }

// #[derive(Deserialize)]
// pub struct RegisterTemp {
//     pub email: String,
//     pub password: String,
//     pub last_name: String,
//     pub first_name: String,
//     pub tel_number: Option<String>,
//     pub address: Option<String>,
// }
