#[macro_use] extern crate lazy_static;

pub mod settings;
pub mod aws_s3;
pub mod hash;
pub mod mailer;
pub mod logger;
pub mod verification;
pub mod time_utils;


#[macro_export]
macro_rules! httperror {
	($error:expr) => {

		HttpResponse::Ok()
			.body(json!({
				"successful": false,
				"error": $error
			}).to_string())

	};
}

#[macro_export]
macro_rules! httpbr {
	($error:expr) => {

		HttpResponse::BadRequest()
			.body(json!({
				"successful": false,
				"error": $error
			}).to_string())
			
	};
}