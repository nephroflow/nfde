use clap::{Args, Parser, Subcommand};
mod database_command;
mod docker_command;
mod healthcheck;

use database_command::handle_database_command;
use docker_command::handle_docker_command;

#[derive(Parser, Debug)]
#[clap(
    author = "Aaron Hallaert",
    version,
    about = "Manage your docker containers and databases"
)]

struct NfdeArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Database(DatabaseCommand),
    Docker(DockerCommand),
}

#[derive(Debug, Args)]
pub struct DatabaseCommand {
    pub action: String,
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct DockerCommand {
    pub action: String,
    pub name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    healthcheck::run()?;

    let args = NfdeArgs::parse();

    match args.action {
        Action::Database(database_command) => handle_database_command(database_command),
        Action::Docker(docker_command) => handle_docker_command(docker_command),
    }
}
