
pub struct Timer {
	pub id: i32,
	pub name: String,
	pub active: bool,
	pub create_date: Option<std::time::SystemTime>,
	pub start_time: Option<i64>,
	pub end_time: Option<i64>,
}

pub fn fetch_timers(project_id: Option<i32>) -> Vec<Timer> {
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

pub fn fetch_active_timers() -> Vec<Timer> {
	let mut timers: Vec<Timer> = Vec::new();

	timers.push(Timer {
		id: 1,
		name: String::from("Timer 1"),
		active: true,
		create_date: Some(std::time::SystemTime::now()),
		start_time: Some(std::time::Duration::from_secs(13*60*60).as_secs() as i64),
		end_time: None,
	});

	return timers;
}

pub fn add_timer(project_id: Option<i32>, payload: Option<Timer>) -> Timer {
	if project_id.is_some() {
		println!("Project ID: {}", project_id.unwrap());
	}

	// TODO - Set default values (id, name, etc)

	let timer = match payload {
		Some(t) => Timer {
			id: 1,
			name: String::from("Timer 1"),
			active: true,
			create_date: Some(std::time::SystemTime::now()),
			start_time: Some(std::time::Duration::from_secs(13*60*60).as_secs() as i64),
			end_time: None,
			..t
		},
		None => Timer {
			id: 1,
			name: String::from("Timer 1"),
			active: true,
			create_date: Some(std::time::SystemTime::now()),
			start_time: Some(std::time::Duration::from_secs(13*60*60).as_secs() as i64),
			end_time: None,
		}
	};

	return timer;
}
