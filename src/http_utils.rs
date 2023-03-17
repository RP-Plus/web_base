
use std::net::SocketAddr;
use std::str::FromStr;
use actix_web::{HttpRequest};

pub trait HttpUtils {

	fn ip_addr(&self) -> Result<String, String>;
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

	fn ip_addr(&self) -> Result<String, String> {

		let info = self.connection_info();
		if let Some(ip_temp) = info.realip_remote_addr() {

			let mut temp: String = String::from(ip_temp);
			temp.push_str(":9999");

			return match temp.parse::<SocketAddr>() {

				Ok(address) => Ok(address.ip().to_string()),
				Err(_msg) => Err("Invalid ip".to_string()) 

			};

		}
		Err("Unable to determin ip address".to_string()) 

	}

}