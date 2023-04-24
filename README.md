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

- backup_image_path
- backup_database_path

## Binaries

### nfde

```bash
nfde -h
```

### run_api

```bash
run_api -h
```

## Installation

Install cargo and run

```bash
cargo install --path .
```
