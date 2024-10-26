use std::env;
use std::path::Path;
use dotenv::dotenv;
use log::error;
use sqlite::Connection;

// Estructura para manejar la conexión a la base de datos
pub struct Database {
    connection: Option<Connection>,
}

impl Database {
    pub fn new() -> Database {
        dotenv().ok();

        if let Ok(db_url) = env::var("DB_URL") {
            let path = Path::new(&db_url);

            if !path.exists() {
                error!("DB file does not exist.");
                return Database { connection: None };
            }

            if let Ok(conn) = Connection::open(path) {
                return Database { connection: Some(conn) };
            }

            error!("Failed to open the database connection.");
            return Database { connection: None };
        }

        error!("DATABASE_URL not set.");

        Database { connection: None }
    }

    pub fn connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }
}