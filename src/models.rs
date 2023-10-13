pub mod models {
    use mongodb::bson::oid::ObjectId;
    use mongodb::bson::DateTime;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MoonDocument {
        pub _id: ObjectId,
        pub lastConnected: DateTime,
        pub macAddress: String,
        pub configData: Option<HashMap<String, String>>,
    }

    #[allow(non_snake_case)]
    #[derive(Clone, Deserialize)]
    pub struct MAC {
        pub macAddress: String,
    }

    #[allow(non_snake_case)]
    #[derive(Clone, Deserialize)]
    pub struct UpdateConfig {
        pub _id: String,
        pub configData: serde_json::Value,
    }
}
