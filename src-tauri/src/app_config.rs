use std::path::PathBuf;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "onemoretime";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub repeat_shortcut: String,
}

impl AppConfig {
    pub fn get_configuration_file_path() -> anyhow::Result<PathBuf> {
        confy::get_configuration_file_path(APP_NAME, None)
            .map_err(|err| { anyhow!("Cannot get configuration file path: {:?}", err)})
    }

    pub fn load() -> anyhow::Result<AppConfig> {
        confy::load::<AppConfig>(APP_NAME, None)
            .map_err(|err| { anyhow!("Cannot load configuration file: {:?}", err)})
    }

    #[allow(dead_code)]
    pub fn save(app_config: &AppConfig) -> anyhow::Result<()> {
        confy::store(APP_NAME, None, app_config)
            .map_err(|err| { anyhow!("Cannot store configuration: {:?}", err)})
    }

}


impl Default for AppConfig {
    fn default() -> Self {
        Self {
            repeat_shortcut: "C-t".to_string()
        }
    }
}
