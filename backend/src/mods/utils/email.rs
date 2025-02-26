use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub fn send_validation_email(user_email: &str, token: &str) -> Result<(), String> {
    let email = Message::builder()
        .from("Ton Site <no-reply@tonsite.com>".parse().unwrap())
        .to(user_email.parse().unwrap())
        .subject("Validation de votre compte")
        .body(format!(
            "Cliquez sur ce lien pour valider votre compte : https://ton-site.com/validate/{}
            \n Attention, ce lien expirera dans 1 heure.",
            token
        ))
        .unwrap();

    let creds = Credentials::new("axellefouq@hotmail.fr".to_string(), "@Charlie_Asher96".to_string());

    let mailer = SmtpTransport::relay("outlook.live.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Erreur envoi email : {:?}", e)),
    }
}
