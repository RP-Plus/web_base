
use std::str::FromStr;
use actix_web::{HttpRequest};

pub trait HttpUtils {

	fn path_var<T>(&self, path_var: &str) -> T where T: FromStr, <T as FromStr>::Err: std::fmt::Debug;

}

impl HttpUtils for HttpRequest {

	fn path_var<T>(&self, path_var: &str) -> T where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {

		self.match_info().get(path_var).unwrap().parse().unwrap()

	}

}