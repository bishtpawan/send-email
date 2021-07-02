use lettre::{SmtpClient, Transport, smtp, SmtpTransport, ClientSecurity};
use lettre_email::Email;
use lettre::smtp::authentication::Credentials;

static SUBJECT: &str = "Conference Mail";
static BODY: &str = "You were absent";
static SUCCESS: &str = "Email Sent";
static FAILURE: &str = "Invalid Email";

pub fn send_email(sender: &str, receiver: &str) -> &'static str {

    match Email::builder()
        .to(receiver)
        .from(sender)
        .subject(SUBJECT)
        .body(BODY)
        .build() {
        Ok(email) => {
            let mailer: SmtpClient = SmtpClient::new_unencrypted_localhost().unwrap();
            let a = mailer.transport().send(email.clone().into()).unwrap();
             println!("{:?}", a);

/*
            let mut transport = smtp::SmtpClient::new(("smtp.gmail.com", smtp::SUBMISSION_PORT),
                                                     ClientSecurity::Required()).unwrap()
                .credentials(Credentials::new("pawanbisht62@gmail.com".to_string(),
                             "applications".to_string())).transport();*/
           /* let mut transport = smtp::SmtpTransportBuilder::new(("smtp.gmail.com", smtp::SUBMISSION_PORT))
                .expect("Failed to create transport")
                .credentials("pawanbisht62@gmail.com".to_string(),
                             "123".to_string())
                .build();*/
            //println!("{:?}", transport.send(email.into()));


            SUCCESS

        }
        Err(_) => FAILURE
    }
}

#[cfg(test)]
mod test {
    use crate::email_service::{FAILURE, send_email, SUCCESS};

    static SENDER: &str = "pawanbisht62@gmail.com";
    static INVALID_SENDER: &str = "pawan@gmail";
    static RECEIVER: &str = "pawanbisht62@gmail.com";

    #[test]
    fn test_send_email_success() {
        assert_eq!(send_email(SENDER, RECEIVER), SUCCESS);
    }

    #[test]
    fn test_send_email_failure() {
        assert_eq!(send_email(INVALID_SENDER, RECEIVER), FAILURE);
    }
}
