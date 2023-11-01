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
	Ls(List)
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

// #[arg(short = 'd', long = "digits")]

pub fn run () {
	let cli = Cli::parse();

	match &cli.command {
		Some(Commands::Ls(name)) => {
			match name.list_type {
				Some(ref _name) => {
					list::list(_name);
				}
				None => {
					println!("Please specificy: projects | timers");
				}
			}
		}
		None => {}
	}
}
