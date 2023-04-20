use clap::{Args, Parser, Subcommand};
mod database_command;
mod docker_command;

use database_command::handle_database_command;
use docker_command::handle_docker_command;
use lib::{
    config::{self, Config},
    healthcheck,
};

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
    Config,
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
    let args = NfdeArgs::parse();

    match args.action {
        Action::Database(database_command) => {
            healthcheck::run()?;
            handle_database_command(database_command)
        }
        Action::Docker(docker_command) => {
            healthcheck::run()?;
            handle_docker_command(docker_command)
        }
        Action::Config => handle_config_command(),
    }
}

fn handle_config_command() -> anyhow::Result<()> {
    println!("Configuring nfde...");
    println!(
        "Configuration file: {}",
        confy::get_configuration_file_path("nfde", None)
            .unwrap()
            .display()
    );
    let default_config = config::get_config()?;

    let nephroflow_database_name = dialoguer::Input::new()
        .with_prompt("Nephroflow database name")
        .default(default_config.nephroflow_database_name)
        .interact_text()?;
    let backup_database_path = dialoguer::Input::new()
        .with_prompt("Backup database path")
        .default(default_config.backup_database_path)
        .interact_text()?;

    let api_image_name = dialoguer::Input::new()
        .with_prompt("API image name")
        .default(default_config.api_image_name)
        .interact_text()?;
    let api_container_name = dialoguer::Input::new()
        .with_prompt("API container name")
        .default(default_config.api_container_name)
        .interact_text()?;
    let backup_image_path = dialoguer::Input::new()
        .with_prompt("Backup image path")
        .default(default_config.backup_image_path)
        .interact_text()?;



    let config = Config {
        api_container_name,
        api_image_name,
        backup_image_path,
        backup_database_path,
        nephroflow_database_name,
    };

    confy::store("nfde", None, config)
        .map_err(|e| anyhow::anyhow!("Failed to write the config file: {}", e))?;

    println!("Configuration has been stored");

    Ok(())
}
