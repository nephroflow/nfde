# NephroFlow Development Environment Manager

Switch NephroFlow development environments easily.

Dump and restore database.
Save and load docker images.

## Prerequisites

- docker
- drop_db
- create_db
- pg_dump
- pg_restore
- (connection to the postgres database from localhost)

## Config

```bash
nfde config
```

### Docker image, container and database names

- api_container_name (default: "web")
- api_image_name (default: "nephroflow/server")
- nephroflow_database_name (default: "nephroflow_development")

### Backup paths

Please provide an absolute path without `~` or `$HOME`

- backup_image_path
- backup_database_path

## Binaries

### nfde

```bash
nfde -h
```

- Docker commands:

  - `save name`: save the current `nephroflow/server` image to a tar file `name.tar`
  - `load`: load one of the tar files to a new `nephroflow/server` image

- Database commands:

  - `dump name`: dump the current database `nephroflow_development`
    to an sql file `name.sql`
  - `restore`: restore one of the databases to the `nephroflow_development` image

### run_api

```bash
run_api -h
```

## Installation

Install cargo and run

```bash
cargo install --path .
```
