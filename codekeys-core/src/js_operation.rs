use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum JsOperation {
    ReloadConfig,
    ReloadPlugins,
    UnloadPlugin { plugin_id: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_js_operation() {
        let json = r#"{ "ReloadConfig": null }"#;
        let op: JsOperation = serde_json::from_str(json).unwrap();
        assert!(matches!(op, JsOperation::ReloadConfig));

        let json = r#"{ "ReloadPlugins": null }"#;
        let op: JsOperation = serde_json::from_str(json).unwrap();
        assert!(matches!(op, JsOperation::ReloadPlugins));

        let json = r#"{ "UnloadPlugin": { "plugin_id": "example-plugin" } }"#;
        let op: JsOperation = serde_json::from_str(json).unwrap();
        match op {
            JsOperation::UnloadPlugin {
                plugin_id: plugin_id,
            } => assert_eq!(plugin_id, "example-plugin"),
            _ => panic!("Expected UnloadPlugin"),
        }
    }
}
