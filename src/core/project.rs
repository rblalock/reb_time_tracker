
pub struct Project {
	pub id: i32,
	pub name: String,
	pub description: String
}

pub fn fetch_projects() -> Vec<Project> {
	let mut projects: Vec<Project> = Vec::new();

	projects.push(Project {
		id: 1,
		name: String::from("Project 1"),
		description: String::from("This is the first project")
	});

	projects.push(Project {
		id: 2,
		name: String::from("Project 2"),
		description: String::from("This is the second project")
	});

	projects
}
