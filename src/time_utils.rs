
use chrono::prelude::*;

fn time_format(amount: i64, amount_str: &str) -> String {

	if amount == 1 {
		return format!("{} {} ago", amount, amount_str);
	}
	return format!("{} {}s ago", amount, amount_str);

}

pub trait TimeUtils {
	fn into_time_since(self) -> String;
	fn into_time_since_short(self) -> String;
}

impl TimeUtils for DateTime<Utc> {

	fn into_time_since(self) -> String {

		let utc: DateTime<Utc> = Utc::now();
		let diff = utc - self;

		if diff.num_weeks() > 0 {

			if diff.num_weeks() < 52 {
				return self.date_naive().to_string();
			} else {
				return time_format(diff.num_weeks(), "week");
			}

		} else if diff.num_days() > 0 {

			return time_format(diff.num_days(), "day");

		} else if diff.num_hours() > 0 {

			return time_format(diff.num_hours(), "hour");

		} else if diff.num_minutes() > 0 {

			return time_format(diff.num_minutes(), "minute");

		} else {

			return time_format(diff.num_seconds(), "second");

		}

	}

	fn into_time_since_short(self) -> String {

		let utc: DateTime<Utc> = Utc::now();
		let diff = utc - self;

		if diff.num_weeks() > 0 {

			if diff.num_weeks() < 52 {
				return self.date_naive().to_string();
			} else {
				return format!("{}w", diff.num_weeks());
			}

		} else if diff.num_days() > 0 {

			return format!("{}d", diff.num_days());

		} else if diff.num_hours() > 0 {

			return format!("{}h", diff.num_hours());

		} else if diff.num_minutes() > 0 {

			return format!("{}m", diff.num_minutes());

		} else {

			return format!("{}s", diff.num_seconds());

		}

	}

}