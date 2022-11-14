use std::ops::Add;
use chrono::{NaiveDate, Local, Duration};

pub struct Birthday {

	pub converted_birthday: NaiveDate

}

impl Birthday {

	pub fn new(birthday: &String) -> Birthday {

		Birthday {
			converted_birthday: NaiveDate::parse_from_str(birthday, "%d:%m:%Y").unwrap()
		}

	}

	pub fn is_at_least_18(&self) -> bool {

		let temp =  self.converted_birthday.add(Duration::weeks(52*18));
		let local = Local::now().date().naive_local();

		return temp < local;

	}

}