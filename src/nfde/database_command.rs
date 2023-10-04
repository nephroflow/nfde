use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

use anyhow::bail;
use command_macros::cmd;
use lib::{config::Config, nf_container_api};
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

use crate::{DatabaseAction, DatabaseCommand};

fn config() -> Config {
    lib::config::get_config().unwrap()
}

pub fn handle_database_command(database_command: DatabaseCommand) -> anyhow::Result<()> {
    if nf_container_api::stop_rails_server().is_ok() {
        println!("Stopped rails server")
    };

    match database_command.database_action {
        DatabaseAction::Remove(remove_command) => remove(remove_command.name),
        DatabaseAction::Dump(dump_command) => dump(dump_command.name),
        DatabaseAction::Restore(restore_command) => restore(restore_command.name),
    }
}

fn remove(name: Option<String>) -> anyhow::Result<()> {
    let db_path = determine_database_path(name)?;

    match cmd!(rm((&db_path.display()))).status() {
        Ok(_) => println!("Removed database dump: {}", &db_path.display()),
        Err(_) => bail!("Could not remove database dump"),
    };

    Ok(())
}

fn dump(name: Option<String>) -> anyhow::Result<()> {
    let db_path = match name {
        Some(name) => Path::new(&config().db_folder()).join(format!("{}.sql", name)),
        None => bail!("Please provide a name for the database dump"),
    };
    println!("Dumping to {}", &db_path.display());

    dump_db(&db_path)?;

    Ok(())
}

fn restore(name: Option<String>) -> anyhow::Result<()> {
    let db_path = determine_database_path(name)?;
    println!("Restoring database from {}", &db_path.display());
    drop_db().ok();
    create_db()?;
    restore_db(&db_path)?;

    println!("Successfully restored database {}", &db_path.display());
    Ok(())
}

fn drop_db() -> anyhow::Result<()> {
    let ran = {
        let mut cmd = ::std::process::Command::new("dropdb");
        cmd.arg("-h");
        cmd.arg("localhost");
        cmd.arg("-U");
        cmd.arg("postgres");
        cmd.arg(&config().nephroflow_database_name);
        cmd
    }
    .status()
    .unwrap()
    .success();

    if ran {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Could not drop database"))
    }
}

fn create_db() -> anyhow::Result<()> {
    let ran = {
        let mut cmd = ::std::process::Command::new("createdb");
        cmd.arg("-h");
        cmd.arg("localhost");
        cmd.arg("-U");
        cmd.arg("postgres");
        cmd.arg(&config().nephroflow_database_name);
        cmd
    }
    .status()
    .unwrap()
    .success();

    if ran {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Could not create database"))
    }
}

fn restore_db(filepath: &Path) -> anyhow::Result<()> {
    let ran = {
        let mut cmd = ::std::process::Command::new("pg_restore");
        cmd.arg("-h");
        cmd.arg("localhost");
        cmd.arg("-U");
        cmd.arg("postgres");
        cmd.arg("-d");
        cmd.arg(&config().nephroflow_database_name);
        cmd.arg("--no-owner");
        cmd.arg("--role=postgres");
        cmd.arg(filepath);
        cmd
    }
    .status()
    .unwrap()
    .success();

    if ran {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Could not restore database"))
    }
}

fn dump_db(filepath: &Path) -> anyhow::Result<()> {
    let ran = {
        let mut cmd = ::std::process::Command::new("pg_dump");
        cmd.arg("-h");
        cmd.arg("localhost");
        cmd.arg("-U");
        cmd.arg("postgres");
        cmd.arg("--file");
        cmd.arg(filepath);
        cmd.arg("--format=c");
        cmd.arg(&config().nephroflow_database_name);
        cmd
    }
    .status()
    .unwrap()
    .success();

    if ran {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Could not dump database"))
    }
}

fn determine_database_path(name: Option<String>) -> anyhow::Result<PathBuf> {
    let db_path = match name {
        Some(name) => Path::new(&config().db_folder()).join(format!("{}.sql", name)),
        None => {
            let selected_file = select_database();
            match selected_file {
                Ok(file) => Path::new(&config().db_folder()).join(file),
                Err(e) => bail!(e),
            }
        }
    };

    // check if file exists
    if !&db_path.exists() {
        bail!("File does not exist: {}", &db_path.display());
    }

    //check if file extension is sql
    if !&db_path.display().to_string().ends_with(".sql") {
        bail!("File is not a sql file: {}", &db_path.display());
    }

    Ok(db_path)
}

fn select_database() -> anyhow::Result<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let files_in_folder = std::fs::read_dir(&config().db_folder()).unwrap();

    let joined_by_newline = files_in_folder
        .filter(|file| {
            file.as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .ends_with(".sql")
        })
        .map(|file| file.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let item_reader = SkimItemReader::default();

    let items = item_reader.of_bufread(Cursor::new(joined_by_newline));

    let skim_output = Skim::run_with(&options, Some(items))
        .ok_or_else(|| anyhow::anyhow!("Skim failed"))
        .unwrap();

    if skim_output.is_abort {
        return Err(anyhow::anyhow!("Database selection aborted"));
    }

    let selected_filename = skim_output
        .selected_items
        .get(0)
        .unwrap()
        .output()
        .to_string();

    Ok(selected_filename)
}
