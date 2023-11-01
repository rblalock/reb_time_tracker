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
	Status(Status)
}

#[derive(Args)]
struct List {
	/// projects | timers
	#[command(subcommand)]
	list_type: Option<ListType>,
}

#[derive(Args)]
struct Status {
	/// Timer status
	#[command(subcommand)]
	status_type: Option<StatusType>,
}

#[derive(Subcommand, Debug)]
pub enum ListType {
	#[command(short_flag = 'p', about = "List projects")]
	Projects,
	#[command(short_flag = 't', about = "List timers")]
	Timers,
}

#[derive(Subcommand, Debug)]
pub enum StatusType {
	#[command(short_flag = 'a', about = "Active timers")]
	Active,
	#[command(short_flag = 'z', about = "All timers")]
	All,
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
		None => {}
	}
}
