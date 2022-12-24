
use chrono::prelude::*;

fn time_format(amount: i64, amount_str: &str) -> String {

	if amount == 1 {
		return format!("{} {} ago", amount, amount_str);
	}
	return format!("{} {}s ago", amount, amount_str);

}

pub fn get_time_since(timestamp: DateTime<Utc>) -> String {

	let utc: DateTime<Utc> = Utc::now();
	let diff = utc - timestamp;

	if diff.num_weeks() > 0 {

		if diff.num_weeks() < 52 {

			return timestamp.date_naive().to_string();

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