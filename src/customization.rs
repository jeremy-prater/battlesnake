use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Customization {
    pub apiversion: String,
    pub author: String,
    pub color: String,
    pub head: String,
    pub tail: String,
    pub version: String,
}

impl Default for Customization {
    fn default() -> Self {
        Customization {
            apiversion: "1".to_string(),
            author: "silverevo".to_string(),
            color: "#888888".to_string(),
            head: "default".to_string(),
            tail: "default".to_string(),
            version: format!("0.0.1-{}", env!("VERGEN_GIT_DESCRIBE")),
        }
    }
}
