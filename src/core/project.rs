use crate::core::timer::Timer;

use crate::core::timer::fetch_timers;

pub struct Project {
	pub id: i32,
	pub name: String,
	pub description: Option<String>,
	pub timers: Option<Vec<Timer>>,
}

pub fn fetch_projects() -> Vec<Project> {
	let mut projects: Vec<Project> = Vec::new();

	projects.push(Project {
		id: 1,
		name: String::from("Project 1"),
		description: Some(String::from("This is the first project")),
		timers: Some(fetch_timers(1))
	});

	projects.push(Project {
		id: 2,
		name: String::from("Project 2"),
		description: None,
		timers: None,
	});

	return projects;
}
