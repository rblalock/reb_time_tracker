use tabled::{
	builder::Builder, 
	settings::Style,
	settings::Padding,
};
use owo_colors::OwoColorize;

use crate::cmd::main::ViewType;
use crate::core;

pub fn view(name: &ViewType) {
	match name {
		ViewType::Project => {
			view_project();
		}
		ViewType::Timer => {
			println!("Timers");
		}
	}
}

pub fn view_project() {
	let project = core::project::fetch_project();

	let mut builder = Builder::default();
	builder.set_header(["#", "Name", "Description", "Active Timers", "Tags", "Rate", "Budget", "Path"]);

	let active_timers = project.timers.as_ref()
		.map(|timers| timers.iter().filter(|timer| timer.active).count())
		.unwrap_or(0);

	builder.push_record([
		project.id.to_string(),
		project.name.green().to_string(),
		project.description.unwrap_or(String::from("")),
		active_timers.bright_red().to_string(),
		project.tags.unwrap_or(Vec::new()).join(", "),
		project.hourly_rate.unwrap_or(0).to_string(),
		project.budget.unwrap_or(0).to_string(),
		project.dir.unwrap_or(String::from("")),
	]);

	let mut table = builder.build();
	table.with(Style::modern());
	table.with(Padding::new(1, 1, 0, 0));

	println!("{}", table);
}
