
use std::str::FromStr;
use actix_web::{HttpRequest};

pub trait HttpUtils {

	fn path_var<T>(&self, path_var: &str) -> T where T: FromStr, <T as FromStr>::Err: std::fmt::Debug;
	fn try_path_var<T>(&self, path_var: &str) -> Result<T, String> where T: FromStr, <T as FromStr>::Err: std::fmt::Debug;

}

impl HttpUtils for HttpRequest {

	fn path_var<T>(&self, path_var: &str) -> T where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {

		self.match_info().get(path_var).unwrap().parse().unwrap()

	}

	fn try_path_var<T>(&self, path_var: &str) -> Result<T, String> where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {

		match self.match_info().get(path_var).unwrap().parse() {
			Ok(var) => return Ok(var),
			Err(_err) => Err(format!("Unable to parse {}", path_var))
		}

	}

}