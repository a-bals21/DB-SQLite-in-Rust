mod dao;

use dao::dao_persona::{PersonaDAO, PersonaDaoImpl};

fn main() {
    env_logger::init();

    let dao_persona = PersonaDaoImpl;

    let personas = dao_persona.get_personas();

    for persona in personas {
        if let Some(nombre) = persona.nombre() {
            print!("\t{}", nombre)
        };

        if let Some(edad) = persona.edad() {
            if *edad > 17i64 { print!(" es mayor de edad") } else { print!(" no es mayor de edad") }
        }

        print!("\n");
    }
}
