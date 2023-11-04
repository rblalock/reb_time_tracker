use owo_colors::OwoColorize;

use crate::cmd::main::AddType;
use crate::core;

pub fn add(name: &AddType) {
	match name {
		AddType::Project { id } => {
			add_timer(Some(id.clone()));
		}
		AddType::Timer => {
			println!("Timers");
		}
	}
}

pub fn add_timer(project_id: Option<i32>) {
	let new_timer = core::timer::add_timer(None, {
		Some(core::timer::Timer {
			id: 123,
			name: "Hello world".to_string(),
			active: true,
			create_date: None,
			start_time: None,
			end_time: None,
		})
	});

	println!("{} created", new_timer.name.green());
}
