use tabled::{
	builder::Builder, 
	settings::Style,
	settings::Padding,
};
use owo_colors::OwoColorize;
use chrono::*;

use crate::cmd::main::StatusType;
use crate::core;

pub fn status(name: &StatusType) {
	match name {
		StatusType::Active => {
			active_table();
		}
		StatusType::All => {
			println!("All timers");
		}
	}
}

fn active_table() {
	let active_timers = core::timer::fetch_active_timers();

	let mut builder = Builder::default();
	builder.set_header(["#", "Name", "Start", "Duration", "Created on"]);

	for timer in active_timers {
		let start = timer.start_time
			.map(|start_time| {
				let hours = start_time / 3600;
				let minutes = (start_time % 3600) / 60;
				let seconds = start_time % 60;

				format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
			})
			.unwrap_or(String::from(""));

		let created_at = timer.create_date
			.map(|create_date| {
				let datetime = chrono::DateTime::<chrono::Utc>::from(create_date);
				let formatted_date = datetime.format("%m/%d/%Y").to_string();
				
				return formatted_date;
			})
			.unwrap_or(String::from(""));

		builder.push_record([
			timer.id.to_string(),
			timer.name.green().to_string(),
			start,
			"01:23:00".bright_red().to_string(),
			created_at,
		]);
	}

	let mut table = builder.build();
	table.with(Style::modern());
	table.with(Padding::new(1, 1, 0, 0));

	println!("{}", table);
}
