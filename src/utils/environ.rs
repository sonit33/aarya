use std::env;

pub struct Environ {
    pub mongo_connection_string: String,
    pub mongo_db: String,
}

impl Default for Environ {
    fn default() -> Self {
        let mongo_connection_string = env::var("MONGO_URL").expect("Missing mongo connection string");
        let mongo_db = env::var("DEFAULT_DB").expect("Missing mongo database name");
        Environ { mongo_connection_string, mongo_db }
    }
}