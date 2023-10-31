use ::clap::{Parser, Subcommand, Args};

mod cmd;

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
    List(List)
}

#[derive(Args)]
struct List {
    /// projects | timers
    string: Option<String>,
}

// #[arg(short = 'd', long = "digits")]

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List(name)) => {
            match name.string {
                Some(ref _name) => {
                    cmd::list::list(_name.to_string());
                }
                None => {
                    println!("Please specificy: projects | timers");
                }
            }
        }
        None => {}
    }
}
