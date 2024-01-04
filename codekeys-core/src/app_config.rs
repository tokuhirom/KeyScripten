use crate::APP_NAME;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    // key is the plugin id.
    // values are map of configurations.
    pub plugins: Option<HashMap<String, PluginConfig>>,
    pub log_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginConfig {
    pub enabled: bool,
    pub config: Option<HashMap<String, String>>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            config: None,
        }
    }
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

    pub fn save(&self) -> anyhow::Result<()> {
        let path = AppConfig::get_configuration_file_path();
        fs::create_dir_all(path.parent().unwrap())?;
        let json = serde_json::to_string(self)?;

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        f.write_all(json.as_bytes())?;
        Ok(())
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
