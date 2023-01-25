#[macro_use] extern crate lazy_static;

pub mod settings;
pub mod aws_s3;
pub mod hash;
pub mod mailer;
pub mod logger;
pub mod verification;
pub mod time_utils;
pub mod db;

#[macro_export]
macro_rules! http_error {
	($error:expr) => {

		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(json!({
				"successful": false,
				"error": $error
			}).to_string())

	};
}

#[macro_export]
macro_rules! http_br {
	($error:expr) => {

		HttpResponse::BadRequest()
			.content_type(ContentType::json())
			.body(json!({
				"successful": false,
				"error": $error
			}).to_string())
			
	};
}

#[macro_export]
macro_rules! http_unauth {
	($error:expr) => {

		HttpResponse::Unauthorized()
			.content_type(ContentType::json())
			.body(json!({
				"successful": false,
				"error": $error
			}).to_string())
			
	};
}

#[macro_export]
macro_rules! http_ise {
	($error:expr) => {

		HttpResponse::InternalServerError()
			.content_type(ContentType::json())
			.body(json!({
				"successful": false,
				"error": $error
			}).to_string())
			
	};
}

#[macro_export]
macro_rules! http_success {
	($json_message:expr) => {

		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(json!({
				"successful": true,
				"message": $json_message
			}).to_string())
			
	};
}