use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use log::error;
use sqlite::{Connection, Result};

struct Database {
    connection: Option<Connection>,
}

impl Database {
    fn new(path: &str) -> Database {
        let path = Path::new(&path);

        if !path.exists() {
            error!("DB file does not exist.");
            return Database { connection: None };
        }

        match Connection::open(path) {
            Ok(conn) => Database { connection: Some(conn) },
            Err(_) => Database { connection: None },
        }
    }

    pub fn get_connection(&mut self) -> Option<&Connection> {
        self.connection.as_ref()
    }
}

// Singleton
lazy_static::lazy_static! {
    static ref DB_INSTANCE: Arc<Mutex<Database>> = {
        dotenv::dotenv().ok();
        let db_url: String = std::env::var("DB_URL").expect("No DB_URL environment variable.");

        let db = Database::new(db_url.as_str());
        Arc::new(Mutex::new(db))
    };
}

pub fn get_db() -> Arc<Mutex<Database>> {
    DB_INSTANCE.clone()
}