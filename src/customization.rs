use serde::Serialize;

#[derive(Serialize)]
struct Customization {
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
            apiversion: (),
            author: (),
            color: (),
            head: (),
            tail: (),
            version: (),
        }
    }
}
