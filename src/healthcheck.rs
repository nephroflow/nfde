use which::which;

pub fn run() -> anyhow::Result<()> {
    println!("Running healthchecks...");
    match database_healthcheck() && docker_healthcheck() {
        true => {
            println!("✅ All healthchecks passed");
            Ok(())
        }
        false => {
            println!("❌ Some healthchecks failed");
            Err(anyhow::anyhow!("Some healthchecks failed"))
        }
    }
}

fn database_healthcheck() -> bool {
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
