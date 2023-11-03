use crate::core::timer::Timer;

use crate::core::timer::fetch_timers;

pub struct Project {
	pub id: i32,
	pub name: String,
	pub description: Option<String>,
	pub timers: Option<Vec<Timer>>,
	pub tags: Option<Vec<String>>,
	pub hourly_rate: Option<i32>,
	pub budget: Option<i32>,
	pub dir: Option<String>,
	pub active: Option<bool>
}

pub fn fetch_projects() -> Vec<Project> {
	let mut projects: Vec<Project> = Vec::new();

	projects.push(Project {
		id: 1,
		name: String::from("Project 1"),
		description: Some(String::from("This is the first project")),
		timers: Some(fetch_timers(Some(1))),
		tags: Some(vec![String::from("tag1"), String::from("tag2")]),
		hourly_rate: Some(100),
		budget: Some(1000),
		dir: Some(String::from("/home/user/projects/project1")),
		active: Some(true),
	});

	projects.push(Project {
		id: 2,
		name: String::from("Project 2"),
		description: None,
		timers: None,
		tags: Some(vec![String::from("tag1")]),
		hourly_rate: Some(100),
		budget: Some(1000),
		dir: Some(String::from("/home/user/projects/project1")),
		active: Some(true),
	});

	return projects;
}

pub fn fetch_project() -> Project {
	let project = Project {
		id: 1,
		name: String::from("Project 1"),
		description: Some(String::from("This is the first project")),
		timers: Some(fetch_timers(Some(1))),
		tags: Some(vec![String::from("tag3")]),
		hourly_rate: Some(100),
		budget: Some(1000),
		dir: Some(String::from("~/projects/project1")),
		active: Some(true),
	};

	return project;
}
