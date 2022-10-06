
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use crate::settings;

pub async fn send_support_email(to_email: &str, subject: &str, body: &str) {

	let email = Message::builder()
		.from(format!("<{}>", settings::get_setting("support_email_username")).parse().unwrap())
		.to(format!("<{}>", to_email).parse().unwrap())
		.subject(subject)
		.body(body.to_string())
		.unwrap();

	let creds = Credentials::new(settings::get_setting("support_email_username"), settings::get_setting("support_email_password"));

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&settings::get_setting("smtp_mailer"))
            .unwrap()
            .credentials(creds)
            .build();

	match mailer.send(email).await {
		Ok(_) => {},
		Err(e) => panic!("Could not send email: {:?}", e),
	}

}


pub async fn send_no_reply_email(to_email: &str, subject: &str, body: &str) {

	let email = Message::builder()
		.from(format!("<{}>", settings::get_setting("no_reply_email_username")).parse().unwrap())
		.to(format!("<{}>", to_email).parse().unwrap())
		.subject(subject)
		.body(body.to_string())
		.unwrap();

	let creds = Credentials::new(settings::get_setting("no_reply_email_username"), settings::get_setting("no_reply_email_password"));

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&settings::get_setting("smtp_mailer"))
            .unwrap()
            .credentials(creds)
            .build();

	match mailer.send(email).await {
		Ok(_) => {},
		Err(e) => panic!("Could not send email: {:?}", e),
	}

}

pub async fn send_alert(subject: &str, body: &str) {

	let email = Message::builder()
		.from(format!("<{}>", settings::get_setting("no_reply_email_username")).parse().unwrap())
		.to(format!("<{}>", settings::get_setting("alert_email")).parse().unwrap())
		.subject(subject)
		.body(body.to_string())
		.unwrap();

	let creds = Credentials::new(settings::get_setting("no_reply_email_username"), settings::get_setting("no_reply_email_password"));

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&settings::get_setting("smtp_mailer"))
            .unwrap()
            .credentials(creds)
            .build();

	match mailer.send(email).await {
		Ok(_) => {},
		Err(e) => panic!("Could not send email: {:?}", e),
	}

}