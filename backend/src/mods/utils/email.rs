use std::env;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_verification_email(user_email: &str, token: &str) -> Result<(), String> {
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()); // Par défaut Gmail

    let email = Message::builder()
        .from("No Reply <no-reply@sitesupercool.com>".parse().unwrap())
        .to(user_email.parse().unwrap())
        .subject("Vérification de votre email")
        .body(format!(
            "Cliquez sur ce lien pour valider votre compte : 
            http://localhost:3000/verify_email?token={}
            \n Attention, ce lien expirera dans 15 minutes.",
            token
        ))
        .unwrap();

    let creds = Credentials::new(smtp_user.clone(), smtp_pass.clone());

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            println!("Email envoyé !");
            Ok(())
        }
        Err(e) => Err(format!("Erreur envoi email : {:?}", e)),
    }
}
