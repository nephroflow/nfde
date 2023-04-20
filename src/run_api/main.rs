use clap::Parser;

use lib::healthcheck;
use lib::nf_container_api::execute_on_nephroflow_container;

#[derive(Parser, Debug)]
#[clap(
    author = "Aaron Hallaert",
    version,
    about = "Run commands in the API container"
)]

struct RunApiArgs {
    #[clap(flatten)]
    pub execute_command: ExecuteCommand,
}

#[derive(Debug, Parser)]
pub struct ExecuteCommand {
    #[clap(short, long)]
    pub not_interactive: bool,
    #[clap(last = true)]
    pub command: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    healthcheck::run()?;

    let args = RunApiArgs::parse();

    execute_on_nephroflow_container(
        args.execute_command.command,
        !args.execute_command.not_interactive,
    )?;

    Ok(())
}
