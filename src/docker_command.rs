use crate::DockerCommand;

pub fn handle_docker_command(docker_command: DockerCommand) -> anyhow::Result<()> {
    match docker_command.action.as_str() {
        "save" => save(docker_command.name),
        "load" => load(docker_command.name),
        _ => println!("Unknown action"),
    };

    Ok(())
}

fn save(name: Option<String>) {
    match name {
        Some(name) => println!("Saving docker image {}", name),
        None => println!("Saving docker image"),
    }
}

fn load(name: Option<String>) {
    match name {
        Some(name) => println!("Loading docker image {}", name),
        None => println!("Loading docker image"),
    }
}
