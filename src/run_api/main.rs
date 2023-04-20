use clap::{Args, Parser, Subcommand};

use lib::healthcheck;
use lib::nf_container_api::execute_on_nephroflow_container;

#[derive(Parser, Debug)]
#[clap(
    author = "Aaron Hallaert",
    version,
    about = "Run commands in the API container",
)]

struct RunApiArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Execute(ExecuteCommand),
}

#[derive(Debug, Args)]
pub struct ExecuteCommand {
    #[clap(short, long)]
    pub not_interactive: bool,
    #[clap(last=true)]
    pub command: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    healthcheck::run()?;

    let args = RunApiArgs::parse();

    match args.action {
        Action::Execute(execute_command) => {
            execute_on_nephroflow_container(execute_command.command, !execute_command.not_interactive)?;
       }
    };

    Ok(())
}
