use command_macros::cmd;

use crate::config;

fn container_name() -> String {
    config::get_config().unwrap().api_container_name
}

pub fn stop_rails_server() -> anyhow::Result<()> {
    println!("Stop Rails server");
    let cmd = vec!["pkill".to_string(), "-9".to_string(), "ruby".to_string()];

    if is_nephroflow_container_running() {
        match attach_and_run_nephroflow_container(&cmd, false) {
            Ok(_) => {
                println!("Rails server stopped");
            }
            Err(_) => {
                println!("Rails server not running");
            }
        }
    };

    Ok(())
}

pub fn execute_on_nephroflow_container(
    command: Vec<String>,
    interactive: bool,
) -> anyhow::Result<()> {
    match is_nephroflow_container_running() {
        true => {
            attach_and_run_nephroflow_container(&command, interactive)?;
        }
        false => {
            match exited_nephroflow_container() {
                true => {
                    stop_nephroflow_container()?;
                }
                false => {}
            };

            start_and_run_nephroflow_container(&command, interactive)?;
        }
    };

    Ok(())
}

fn start_and_run_nephroflow_container(command: &[String], interactive: bool) -> anyhow::Result<()> {
    println!("Create and run API container");

    let ran = {
        let mut cmd = ::std::process::Command::new("docker-compose");
        cmd.arg("run");
        if !interactive {
            cmd.arg("-T");
        }
        cmd.arg("--rm");
        cmd.arg("--service-ports");
        cmd.arg("--name");
        cmd.arg(container_name());
        cmd.arg("web");
        command.iter().for_each(|arg| {
            cmd.arg(arg);
        });
        cmd
    }
    .status()
    .unwrap()
    .success();

    if ran {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Command failed to run"))
    }
}

fn attach_and_run_nephroflow_container(
    command: &[String],
    interactive: bool,
) -> anyhow::Result<()> {
    println!("Attach and run API container");

    let ran = {
        let mut cmd = ::std::process::Command::new("docker");
        cmd.arg("exec");
        if interactive {
            cmd.arg("-it");
        }
        cmd.arg(container_name());
        command.iter().for_each(|arg| {
            cmd.arg(arg);
        });
        cmd
    }
    .status()
    .unwrap()
    .success();

    if ran {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Command failed to run"))
    }
}

fn stop_nephroflow_container() -> anyhow::Result<()> {
    // docker rm
    cmd!(docker rm ((container_name()))).status()?;

    Ok(())
}

fn exited_nephroflow_container() -> bool {
    let exited =
        cmd!(docker ps ("-aq")("status=exited") ("-f")  ((format!("name={}", container_name()))))
            .output()
            .unwrap();

    if !exited.stdout.is_empty() {
        println!("Nephroflow container is in exit status");
        true
    } else {
        println!("Nephroflow container is not in exit status");
        false
    }
}

pub fn is_nephroflow_container_running() -> bool {
    let running = cmd!(docker ps ("-q") ("-f") ((format!("name={}", container_name()))))
        .output()
        .unwrap();

    if !running.stdout.is_empty() {
        println!("Nephroflow container is running");
        true
    } else {
        println!("Nephroflow container is not running");
        false
    }
}
