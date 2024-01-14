use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum JsOperation {
    ReloadConfig,
    ReloadPlugins,
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
    }
}
