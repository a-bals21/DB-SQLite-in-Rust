use std::ops::Deref;
use log::error;
use sqlite::{State, Statement};
use crate::dao::db_connection::get_db;

#[allow(unused)]
#[derive(Debug)]
pub struct Persona {
    id: Option<i64>,
    nombre: Option<String>,
    edad: Option<i64>,
}

fn iterar_stmt_persona(stmt: &mut Statement) -> Vec<Persona> {
    let mut personas = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let id: Option<i64> = stmt.read( "id").ok();
        let nombre: Option<String> = stmt.read("nombre").ok();
        let edad: Option<i64> = stmt.read("edad").ok();

        personas.push(Persona {id, nombre, edad});
    }

    personas
}

trait PersonaDAO {
    fn get_persona(&self, id: Option<i64>) -> Option<Persona>;
    fn get_personas(&self) -> Option<Vec<Persona>>;
}

pub struct PersonaDaoImpl;

impl PersonaDAO for PersonaDaoImpl {
    fn get_persona(&self, id: Option<i64>) -> Option<Persona> {
        let db = get_db();
        let guard = db.lock().unwrap();
        let connection = guard.deref().get_connection();

        let mut personas: Vec<Persona> = Vec::new();

        if let Ok(mut stmt) = connection.prepare("select * from Persona where id = :id") {
            stmt.bind((":id", id)).expect("Error al iterar sobre los resultado");

            personas = iterar_stmt_persona(&mut stmt);
        } else {
            error!("Could not connect to persona table.");
        }

        None
    }

    fn get_personas(&self) -> Vec<Persona> {
        let db = get_db();
        let guard = db.lock().unwrap();
        let connection = guard.deref().get_connection();

        let mut personas: Vec<Persona> = Vec::new();

        if let Ok(mut stmt) = connection.prepare("select * from Persona") {
            personas = iterar_stmt_persona(&mut stmt);
            return personas.get(0);
        } else {
            error!("Could not connect to persona table.");
        }

        personas
    }
}