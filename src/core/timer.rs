
pub struct Timer {
	pub id: i32,
	pub name: String,
	pub active: bool,
	pub create_date: Option<std::time::SystemTime>,
	pub start_time: Option<i64>,
	pub end_time: Option<i64>,
}

pub fn fetch_timers(project_id: i32) -> Vec<Timer> {
	let mut timers: Vec<Timer> = Vec::new();

	timers.push(Timer {
		id: 1,
		name: String::from("Timer 1"),
		active: true,
		create_date: Some(std::time::SystemTime::now()),
		start_time: Some(std::time::Duration::from_secs(23*60*60).as_secs() as i64),
		end_time: None,
	});
	timers.push(Timer {
		id: 2,
		name: String::from("Timer 2"),
		active: false,
		create_date: Some(std::time::SystemTime::now()),
		start_time: Some(std::time::Duration::from_secs(21*60*60).as_secs() as i64),
		end_time: Some(std::time::Duration::from_secs(23*60*60).as_secs() as i64),
	});

	return timers;
}
