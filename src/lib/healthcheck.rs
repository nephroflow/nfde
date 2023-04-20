use which::which;
pub fn run() -> anyhow::Result<()> {
    println!("Running healthchecks...");
    match database_healthcheck() && docker_healthcheck() && config_healthcheck() {
        true => {
            println!();
            println!("✅ All healthchecks passed");
            Ok(())
        }
        false => {
            println!();
            println!("Edit config with `nfde config`");
            Err(anyhow::anyhow!("Some healthchecks failed"))
        }
    }
}

fn database_healthcheck() -> bool {
    println!();
    println!("Database healthcheck...");

    let pg_dump_path = which("pg_dump");
    let pg_restore_path = which("pg_restore");

    let pg_dump_found = match pg_dump_path {
        Ok(path) => {
            println!("✅ pg_dump found at: {}", path.display());
            true
        }
        Err(_) => {
            println!("❌ pg_dump not found");
            false
        }
    };

    let pg_restore_found = match pg_restore_path {
        Ok(path) => {
            println!("✅ pg_restore found at: {}", path.display());
            true
        }
        Err(_) => {
            println!("❌ pg_restore not found");
            false
        }
    };

    let createdb_path = which("createdb");
    let dropdb_path = which("dropdb");

    let createdb_found = match createdb_path {
        Ok(path) => {
            println!("✅ createdb found at: {}", path.display());
            true
        }
        Err(_) => {
            println!("❌ createdb not found");
            false
        }
    };

    let dropdb_found = match dropdb_path {
        Ok(path) => {
            println!("✅ dropdb found at: {}", path.display());
            true
        }
        Err(_) => {
            println!("❌ dropdb not found");
            false
        }
    };

    pg_dump_found && pg_restore_found && createdb_found && dropdb_found
}

fn docker_healthcheck() -> bool {
    println!();
    println!("Docker healthcheck...");

    let docker_path = which("docker");

    let docker_found = match docker_path {
        Ok(path) => {
            println!("✅ docker found at: {}", path.display());
            true
        }
        Err(_) => {
            println!("❌ docker not found");
            false
        }
    };

    // check if docker is running
    let docker_running = match std::process::Command::new("docker").arg("ps").output() {
        Ok(_) => {
            println!("✅ docker is running");
            true
        }
        Err(_) => {
            println!("❌ docker is not running");
            false
        }
    };

    docker_found && docker_running
}

fn config_healthcheck() -> bool {
    println!();
    println!("Config healthcheck...");
    let config = crate::config::get_config().unwrap();
    let api_container_name = config.api_container_name;
    let api_image_name = config.api_image_name;
    let backup_image_path = config.backup_image_path;
    let backup_database_path = config.backup_database_path;
    let nephroflow_database_name = config.nephroflow_database_name;

    let api_container_name_found = !api_container_name.is_empty();
    match api_container_name_found {
        true => println!("✅ API container name configured"),
        false => println!("❌ API container name not configured"),
    }


    let api_image_name_found = !api_image_name.is_empty();
    match api_image_name_found {
        true => println!("✅ API image name configured"),
        false => println!("❌ API image name not configured"),
    }

    let backup_image_path_found = !backup_image_path.is_empty() &&
        std::path::Path::new(&backup_image_path).exists();

    match backup_image_path_found {
        true => println!("✅ Backup image path configured and exists"),
        false => println!("❌ Backup image path not configured or does not exist: {}", &backup_image_path),
    }

    let backup_database_path_found = !backup_database_path.is_empty()
        && std::path::Path::new(&backup_database_path).exists();

    match backup_database_path_found {
        true => println!("✅ Backup database path configured and exists"),
        false => println!("❌ Backup database path not configured or does not exist: {}", &backup_database_path),
    }

    let nephroflow_database_name_found = !nephroflow_database_name.is_empty();
    match nephroflow_database_name_found {
        true => println!("✅ Nephroflow database name configured"),
        false => println!("❌ Nephroflow database name not configured"),
    }

    api_container_name_found
        && api_image_name_found
        && backup_image_path_found
        && backup_database_path_found
        && nephroflow_database_name_found
}
