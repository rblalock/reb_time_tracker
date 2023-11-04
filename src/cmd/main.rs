use ::clap::{Parser, Subcommand, Args};

use super::*;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "reb - A cool CLI time tracking tool")]

struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// projects | timers
	Ls(List),
	/// Timer status
	Status(Status),
	// View project | timer details
	View(View),
	// Create a project | timer
	Add(Add),
}

#[derive(Args)]
struct List {
	/// projects | timers
	#[command(subcommand)]
	list_type: Option<ListType>,
}

#[derive(Subcommand, Debug)]
pub enum ListType {
	#[command(short_flag = 'p', about = "List projects")]
	Projects,
	#[command(short_flag = 't', about = "List timers")]
	Timers,
}

#[derive(Args)]
struct Status {
	/// Timer status
	#[command(subcommand)]
	status_type: Option<StatusType>,
}

#[derive(Subcommand, Debug)]
pub enum StatusType {
	#[command(short_flag = 'a', about = "Active timers")]
	Active,
	#[command(short_flag = 'z', about = "All timers")]
	All,
}

#[derive(Args)]
struct View {
	/// View project | timer details
	#[command(subcommand)]
	view_type: Option<ViewType>,
}

#[derive(Subcommand, Debug)]
pub enum ViewType {
	#[command(short_flag = 'p', about = "Project details")]
	Project,
	#[command(short_flag = 'z', about = "Timer details")]
	Timer,
}

#[derive(Args)]
struct Add {
	/// Add project | timer
	#[command(subcommand)]
	add_type: Option<AddType>,
}

#[derive(Subcommand, Debug)]
pub enum AddType {
	#[command(short_flag = 'p', about = "Project")]
	Project {
		#[arg(action)]
		id: i32,
	},
	#[command(short_flag = 't', about = "Timer")]
	Timer,
}

pub fn run () {
	let cli = Cli::parse();

	match &cli.command {
		Some(Commands::Ls(name)) => {
			match name.list_type {
				Some(ref _name) => {
					list::list(_name);
				}
				None => {
					println!("Please specify: projects | timers");
				}
			}
		}
		Some(Commands::Status(name)) => {
			match name.status_type {
				Some(ref _name) => {
					status::status(_name);
				}
				None => {
					println!("Please specify: all | active");
				}
			}
		}
		Some(Commands::View(name)) => {
			match name.view_type {
				Some(ref _name) => {
					view::view(_name);
				}
				None => {
					println!("Please specify: project | timer");
				}
			}
		}
		Some(Commands::Add(name)) => {
			match name.add_type {
				Some(ref _name) => {
					add::add(_name);
				}
				None => {
					println!("Please specify: project | timer");
				}
			}
		}
		None => {}
	}
}
