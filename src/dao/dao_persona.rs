use log::error;
use sqlite::{State, Statement};
use crate::dao::db_connection::Database;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Persona {
    id: Option<i64>,
    nombre: Option<String>,
    edad: Option<i64>,
}

impl Persona {
    pub fn id(&self) -> &Option<i64> {
        return &self.id;
    }

    pub fn nombre(&self) -> &Option<String> {
        return &self.nombre;
    }

    pub fn edad(&self) -> &Option<i64> {
        return &self.edad;
    }
}

fn iterar_stmt_persona(stmt: &mut Statement) -> Vec<Persona> {
    let mut personas: Vec<Persona> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let id: Option<i64> = stmt.read( "id").ok();
        let nombre: Option<String> = stmt.read("nombre").ok();
        let edad: Option<i64> = stmt.read("edad").ok();

        personas.push(Persona { id, nombre, edad });
    }

    personas
}

pub trait PersonaDAO {
    fn get_persona(&self, id: Option<i64>) -> Option<Persona>;
    fn get_personas(&self) -> Vec<Persona>;
}

pub struct PersonaDaoImpl;

impl PersonaDAO for PersonaDaoImpl {
    fn get_persona(&self, id: Option<i64>) -> Option<Persona> {
        let db = Database::new();

        if let Some(conn) = db.connection() {
            if let Ok(mut stmt) = conn.prepare("select * from Persona where id = :id") {
                stmt.bind((":id", id)).expect("Error al iterar sobre los resultado");
                let personas = iterar_stmt_persona(&mut stmt);

                return personas.first().cloned();
            }

        } else {
            error!("Could not connect to persona table.");
        }

        None
    }

    fn get_personas(&self) -> Vec<Persona> {
        let mut personas: Vec<Persona> = Vec::new();
        let db = Database::new();

        if let Some(conn) = db.connection() {
            if let Ok(mut stmt) = conn.prepare("select * from Persona") {
                personas = iterar_stmt_persona(&mut stmt);
            }
        } else {
            error!("Could not connect to persona table.");
        }

        personas
    }
}