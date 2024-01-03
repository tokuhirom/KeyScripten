use crate::APP_NAME;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    // key is the plugin id.
    // values are map of configurations.
    pub plugins: Option<HashMap<String, HashMap<String, String>>>,
    pub log_level: String,
}

impl AppConfig {
    pub fn get_configuration_file_path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join(APP_NAME)
            .join("config.json")
    }

    pub fn load() -> anyhow::Result<AppConfig> {
        let path = AppConfig::get_configuration_file_path();
        log::info!("Loading configuration from {:?}", path);

        match File::open(path.clone()) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(config) => Ok(config),
                Err(err) => {
                    log::error!(
                        "Cannot deserialize configuration file({:?}): {:?}",
                        path,
                        err
                    );
                    Ok(AppConfig::default())
                }
            },
            Err(err) => {
                log::warn!("Cannot open configuration file({:?}): {:?}", path, err);
                // fallback to default configuration
                Ok(AppConfig::default())
            }
        }
    }

    #[allow(dead_code)]
    pub fn save(app_config: &AppConfig) -> anyhow::Result<()> {
        confy::store_path(AppConfig::get_configuration_file_path(), app_config)
            .map_err(|err| anyhow!("Cannot store configuration: {:?}", err))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            plugins: Some(HashMap::new()),
            log_level: "INFO".to_string(),
        }
    }
}
