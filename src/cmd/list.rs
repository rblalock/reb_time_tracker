use tabled::{
	builder::Builder, 
	settings::Style,
	settings::Padding,
};

use crate::cmd::main::ListType;
use crate::core;

pub fn list(name: &ListType) {
	match name {
		ListType::Projects => {
			projects_table();
		}
		ListType::Timers => {
			println!("Timers");
		}
	}
}

fn projects_table() {
	let projects = core::project::fetch_projects();

	let mut builder = Builder::default();
	builder.set_header(["#", "Name", "Description", "Active Timers"]);

	// Count the number of active timers
	for project in projects {
		let active_timers = project.timers.as_ref()
			.map(|timers| timers.iter().filter(|timer| timer.active).count())
			.unwrap_or(0);

		builder.push_record([
			project.id.to_string(),
			project.name,
			project.description.unwrap_or(String::from("")),
			active_timers.to_string(),
		]);
	}

	let mut table = builder.build();
	table.with(Style::modern());
	table.with(Padding::new(1, 1, 0, 0));

	println!("{}", table);
}
