mod dao;

fn main() {
    let dao_persona = dao::dao_persona::PersonaDaoImpl{};

    todo!();
    let personas;

    for persona in personas {
        if let Some(nombre) = persona.nombre {
            print!("\t{}", nombre)
        };

        if let Some(edad) = persona.edad {
            if edad > 17 { print!(" es mayor de edad") } else { print!(" no es mayor de edad") }
        }

        print!("\n");
    }
}
