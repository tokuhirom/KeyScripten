use anyhow::anyhow;
use std::fs;

pub fn add_plugin(name: String, plugin_id: String, description: String) -> anyhow::Result<()> {
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
    write_plugin(plugin_id, content)
}

pub fn write_plugin(plugin_id: String, content: String) -> anyhow::Result<()> {
    let configdir = dirs::config_dir().unwrap();
    let plugins = configdir.join("plugins");
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_add_plugin_success() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Set the HOME environment variable to the temporary directory
        env::set_var("HOME", temp_path);

        // Call the add_plugin function
        let result = add_plugin(
            "TestPlugin".to_string(),
            "test_plugin".to_string(),
            "A test plugin".to_string(),
        );

        // Check if the result is Ok
        assert!(result.is_ok());

        // Verify that the plugin file exists
        let plugin_path = temp_path.join("Library/Application Support/plugins/test_plugin.js");
        assert!(plugin_path.exists());

        // Clean up is handled automatically by the tempdir crate
    }
}
