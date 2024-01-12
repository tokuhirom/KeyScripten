use anyhow::anyhow;
use std::fs;
use std::path::Path;

struct Plugins {
    basedir: String,
}
impl Plugins {
    pub fn new() -> anyhow::Result<Plugins> {
        let configdir = dirs::config_dir().ok_or_else(|| anyhow!("Config directory not found"))?;
        let plugins_dir = configdir.join("plugins");
        Ok(Self::new_with_basedir(
            plugins_dir.to_str().unwrap().to_string(),
        ))
    }

    pub fn new_with_basedir(basedir: String) -> Plugins {
        Plugins { basedir }
    }

    pub fn list(&self) -> anyhow::Result<Vec<String>> {
        let plugins_dir = Path::new(&self.basedir);

        if !plugins_dir.exists() {
            return Ok(vec![]);
        }

        let mut plugin_ids = Vec::new();
        for entry in fs::read_dir(plugins_dir)? {
            let entry = entry?;
            let path = entry.path();
            log::info!("Found {:?}", path);
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "js" {
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(file_stem_str) = file_stem.to_str() {
                                plugin_ids.push(file_stem_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(plugin_ids)
    }

    pub fn add(&self, plugin_id: String, name: String, description: String) -> anyhow::Result<()> {
        let content = format!(
            r#"
    registerPlugin(
        "{}",
        "{}",
        `{}`,
    )
    "#,
            plugin_id, name, description
        );
        self.write(plugin_id, content)
    }

    pub fn write(&self, plugin_id: String, content: String) -> anyhow::Result<()> {
        let plugins = Path::new(&self.basedir);
        if !plugins.exists() {
            fs::create_dir_all(&plugins)
                .map_err(|err| anyhow!("Cannot create plugins directory: {:?}", err))?;
        }
        let pluginpath = plugins.join(format!("{}.js", plugin_id));
        log::info!("Writing new plugin: {:?}", pluginpath);
        fs::write(pluginpath.as_path(), content).map_err(|err| {
            anyhow!(
                "Cannot write new plugin: path={:?}, err={:?}",
                pluginpath,
                err
            )
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    use std::sync::Once;
    static INIT: Once = Once::new();

    pub fn initialize_logger() {
        INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug");
            env_logger::init();
        });
    }

    #[test]
    fn test_add_plugin_success() {
        initialize_logger();

        // Create a temporary directory
        let temp_dir = TempDir::with_prefix("add_plugin").unwrap();
        let temp_path = temp_dir.path();
        let plugins = Plugins::new_with_basedir(temp_path.to_str().unwrap().to_string());

        // Call the add_plugin function
        let result = plugins.add(
            "test_plugin".to_string(),
            "TestPlugin".to_string(),
            "A test plugin".to_string(),
        );

        // Check if the result is Ok
        assert!(result.is_ok());

        // Verify that the plugin file exists
        let plugin_path = temp_path.join("test_plugin.js");
        assert!(plugin_path.exists());

        // Clean up is handled automatically by the tempdir crate
    }

    #[test]
    fn test_list_plugins() {
        initialize_logger();

        let temp_dir = TempDir::with_prefix("list_plugins").unwrap();
        let temp_path = temp_dir.path();
        let plugins = Plugins::new_with_basedir(temp_path.to_str().unwrap().to_string());

        plugins
            .add(
                "plugin_one".to_string(),
                "PluginOne".to_string(),
                "Description one".to_string(),
            )
            .unwrap();
        plugins
            .add(
                "plugin_two".to_string(),
                "PluginTwo".to_string(),
                "Description two".to_string(),
            )
            .unwrap();

        let plugin_ids = plugins.list().unwrap();
        assert_eq!(plugin_ids, vec!["plugin_one", "plugin_two"]);

        // Check that both plugin files exist
        assert!(temp_path
            .join("plugin_one.js")
            .exists());
        assert!(temp_path
            .join("plugin_two.js")
            .exists());
    }
}
