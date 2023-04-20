use std::io::Cursor;

use anyhow::bail;
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

use crate::DatabaseCommand;

const DBFOLDER: &str = "/Users/aaronhallaert/Developer/nephroflow/db_dumps";

pub fn handle_database_command(database_command: DatabaseCommand) -> anyhow::Result<()> {
    match database_command.action.as_str() {
        "dump" => {
            dump(database_command.name)
        },
        "restore" => restore(database_command.name),
        _ => {
            bail!("Unknown database action");
        }
    }
}

fn dump(name: Option<String>) -> anyhow::Result<()>{
    let db_path = match name {
        Some(name) => {
            format!("{}/{}.sql", DBFOLDER, name)
        },
        None => bail!("Please provide a name for the database dump"),
    };
    println!("Dumping to {}", db_path);

    Ok(())
}

fn restore(name: Option<String>) -> anyhow::Result<()> {
    let db_path = determine_database_path(name)?;
    println!("Restoring database from {}", db_path);



    Ok(())
}

fn determine_database_path(name: Option<String>) -> anyhow::Result<String> {
    let db_path = match name {
        Some(name) => {
            format!("{}/{}.sql", DBFOLDER, name)
        }
        None => {
            let selected_file = select_database();
            match selected_file {
                Ok(file) => {
                    format!("{}/{}", DBFOLDER, file)
                }
                Err(e) => bail!(e),
            }
        }
    };

    // check if file exists
    if !std::path::Path::new(&db_path).exists() {
        bail!("File does not exist");
    }

    //check if file extension is sql
    if !db_path.ends_with(".sql") {
        bail!("File is not a sql file");
    }

    Ok(db_path)
}

fn select_database() -> anyhow::Result<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let files_in_folder = std::fs::read_dir(DBFOLDER).unwrap();

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
