use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_container_name: String,
    pub api_image_name: String,
    pub backup_image_path: String,
    pub backup_database_path: String,
    pub nephroflow_database_name: String,
}

pub fn get_config() -> anyhow::Result<Config> {
    let mut config = confy::load::<Config>("nfde", None)?;

    if config.api_container_name.is_empty() {
        config.api_container_name = "web".to_string();
    }

    if config.api_image_name.is_empty() {
        config.api_image_name = "nephroflow/server".to_string();
    }

    if config.nephroflow_database_name.is_empty() {
        config.nephroflow_database_name = "nephroflow_development".to_string();
    }

    Ok(config)
}
