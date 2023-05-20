// use crate::tareas::Estado;
use std::{
    io::{stdin, stdout as stdo, /*&&&BufRead, BufWriter*/ Result, Write},
    vec,
};

use termion::{clear, cursor, input::TermRead, raw::IntoRawMode};

use crate::{
    errores::error_fin,
    tareas::{Estado, IdTareas, Tarea},
    vista::opciones_tareas:: opciones_tarea,
};

pub mod generar {

    use std::fmt::Display;

    const SALIR: u8 = 0;

    pub const SELC_LISTA: &str = "◉";
    pub const NO_SELC_LISTA: &str = "○";
    pub const LISTA_DISP_ARRIBA: &str = "↟";
    pub const LISTA_DISP_ABAJO: &str = "↡";

    pub const SELC_OPCIONES_INICIO: &str = "⇏ ";
    pub const SELC_OPCIONES_FIN: &str = " ⇍";
    pub const SELC_DISP_DERECHA: &str = "↣";
    pub const SELC_DISP_IZQUIERDA: &str = "↢";

    pub const SELECCCION_NO_SELECT: &str = "[]";
    pub const SELECCCION_SELECT: &str = "[V]";
    pub const SELECCCION_POS: &str = ">*";
    pub const NO_SELECCCION_POS: &str = " *";

    pub const POSICION_INICIO: u8 = 0;
    pub const RANGO_INICIO: u8 = 0;
    pub const RANGO_LISTA: u8 = 5;
    pub const RANGO_OPCIONES: u8 = 5;
    pub const RANGO_SELECCION: u8 = 5;

    use crate::errores::error_fin;
    use std::io::{stdin, stdout as stdo, /*BufRead, BufWriter*/ Result, Stdout, Write};
    use termion::{
        clear, cursor,
        event::Key,
        input::TermRead,
        raw::{IntoRawMode, RawTerminal},
    };

    pub fn menu_lista<T: Display>(menus: &Vec<T>, mut posicion: usize) -> usize {
        let mut inicio: usize = RANGO_INICIO as usize;
        let mut rango: usize = RANGO_LISTA as usize;

        let mut stdout: RawTerminal<Stdout>;

        match stdo().into_raw_mode() {
            Result::Ok(stdt) => {
                stdout = stdt;
            }
            Result::Err(error) => {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            }
        };

        stdout
            .write(format!("{}{}{}", cursor::Goto(1, 1), cursor::Hide, clear::All).as_bytes())
            .unwrap_or_else(|error| {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            });

        if posicion >= menus.len() {
            posicion = POSICION_INICIO as usize;
        }

        if posicion > inicio + rango - 1 {
            inicio = posicion - (posicion % rango);
        }

        if inicio + rango > menus.len() {
            rango = menus.len() - inicio;
        }

        if inicio + rango > rango {
            print!("\t{}\n\n\r", LISTA_DISP_ARRIBA);
        } else {
            print!("\n\n");
        }

        for (pos, menu) in menus[inicio..inicio + rango].iter().enumerate() {
            let mut selc: &str = NO_SELC_LISTA;

            if pos + inicio == posicion as usize {
                selc = SELC_LISTA;
            }

            if let Err(error) = stdout.write(format!("{}\t{}\n\r", selc, *menu).as_bytes()) {
                print!("Algo ha fallado al general el menu, {}", error);
                return 0;
            }

            stdout.flush().unwrap_or_else(|_datos| print!("Error"));
        }

        if inicio + rango < menus.len() {
            print!("\n\t{}\n", LISTA_DISP_ABAJO);
        }

        match stdin().keys().next() {
            Some(Result::Ok(Key::Up | Key::Left)) => {
                if posicion > 0 {
                    posicion -= 1;
                } else if posicion == 0 {
                    posicion = menus.len() - 1;
                }
                return menu_lista(menus, posicion);
            }
            Some(Result::Ok(Key::Down | Key::Right)) => {
                posicion += 1;
                return menu_lista(menus, posicion);
            }
            Some(Result::Ok(Key::Char('\n') | Key::Char(_))) => return posicion + 1,
            Some(_) => return SALIR as usize,
            None => return SALIR as usize,
        };
    }

    //Menu para mostrar las opciones de forma paralela
    pub fn menu_opciones<T: Display>(
        mensaje: &str,
        opciones: Vec<T>,
        mut posicion: usize,
    ) -> usize {
        let mut inicio: usize = RANGO_INICIO as usize;
        let mut rango: usize = RANGO_OPCIONES as usize;

        let mut stdout: RawTerminal<Stdout>;

        match stdo().into_raw_mode() {
            Result::Ok(stdt) => {
                stdout = stdt;
            }
            Result::Err(error) => {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            }
        };

        stdout
            .write(format!("{}{}{}", cursor::Goto(1, 1), cursor::Hide, clear::All).as_bytes())
            .unwrap_or_else(|error| {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            });

        if posicion >= opciones.len() {
            posicion = POSICION_INICIO as usize;
        }

        if posicion > inicio + rango - 1 {
            inicio = posicion - (posicion % rango);
        }

        if inicio + rango > opciones.len() {
            rango = opciones.len() - inicio;
        }

        stdout
            .write(format!("\t\t\t{}\r\n\n", mensaje).as_bytes())
            .unwrap();

        if inicio + rango > rango {
            print!("{}", SELC_DISP_IZQUIERDA);
        }

        print!("\t");

        for (pos, menu) in opciones[inicio..inicio + rango].iter().enumerate() {
            let mut selc: &str = "";
            let mut selc_fin: &str = "";

            if pos + inicio == posicion as usize {
                selc = SELC_OPCIONES_INICIO;
                selc_fin = SELC_OPCIONES_FIN;
            }

            if let Err(error) = stdout.write(format!("{}{}{}\t", selc, *menu, selc_fin).as_bytes())
            {
                print!("Algo ha fallado al general el menu, {}", error);
                return 0;
            }

            stdout.flush().unwrap_or_else(|_datos| print!("Error"));
        }

        if inicio + rango < opciones.len() {
            print!("{}", SELC_DISP_DERECHA);
        }

        print!("\n\r");

        match stdin().keys().next() {
            Some(Result::Ok(Key::Up | Key::Left)) => {
                if posicion > 0 {
                    posicion -= 1;
                } else if posicion == 0 {
                    posicion = opciones.len() - 1;
                }
                return menu_opciones(mensaje, opciones, posicion);
            }
            Some(Result::Ok(Key::Down | Key::Right)) => {
                posicion += 1;
                return menu_opciones(mensaje, opciones, posicion);
            }
            Some(Result::Ok(Key::Char('\n') | Key::Char(_))) => return posicion + 1,
            Some(_) => {
                println!("Saliendo con el valor de SALIR {}\r", SALIR);
                return SALIR as usize;
            }
            None => return SALIR as usize,
        }
    }

    pub fn menu_seleccion<T: Display>(
        mensaje: &str,
        opciones: Vec<T>,
        selecciones: &mut Vec<usize>,
        mut posicion: usize,
    ) -> () {
        let mut inicio: usize = RANGO_INICIO as usize;
        let mut rango: usize = RANGO_SELECCION as usize;

        let mut stdout: RawTerminal<Stdout>;

        match stdo().into_raw_mode() {
            Result::Ok(stdt) => {
                stdout = stdt;
            }
            Result::Err(error) => {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            }
        };

        stdout
            .write(format!("{}{}{}", cursor::Goto(1, 1), cursor::Hide, clear::All).as_bytes())
            .unwrap_or_else(|error| {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            });

        if posicion >= opciones.len() {
            posicion = POSICION_INICIO as usize;
        }

        if posicion > inicio + rango - 1 {
            inicio = posicion - (posicion % rango);
        }

        if inicio + rango > opciones.len() {
            rango = opciones.len() - inicio;
        }

        stdout
            .write(format!("\t\t{}\n\n\r", mensaje).as_bytes())
            .unwrap();

        if inicio + rango > rango {
            print!("{}", LISTA_DISP_ARRIBA);
        }

        print!("\n\r");

        let mut selc: &str = SELECCCION_NO_SELECT;
        let mut indicador: &str = NO_SELECCCION_POS;

        for (pos, menu) in opciones[inicio..inicio + rango].iter().enumerate() {
            if selecciones.contains(&(pos + inicio)) {
                selc = SELECCCION_SELECT;
            }

            if pos + inicio == posicion as usize {
                indicador = SELECCCION_POS;
            }

            if let Err(error) =
                stdout.write(format!("{}{}\t{}\n\r", indicador, selc, *menu).as_bytes())
            {
                print!("Algo ha fallado al general el menu, {}", error);
                return;
            }

            stdout.flush().unwrap_or_else(|_datos| print!("Error"));

            selc = SELECCCION_NO_SELECT;
            indicador = NO_SELECCCION_POS;
        }

        if inicio + rango < opciones.len() {
            print!("{}", LISTA_DISP_ABAJO);
        }

        print!("\n\r");

        match stdin().keys().next() {
            Some(Result::Ok(Key::Up | Key::Left)) => {
                if posicion > 0 {
                    posicion -= 1;
                } else if posicion == 0 {
                    posicion = opciones.len() - 1;
                }
                return menu_seleccion(mensaje, opciones, selecciones, posicion);
            }
            Some(Result::Ok(Key::Down | Key::Right)) => {
                posicion += 1;
                return menu_seleccion(mensaje, opciones, selecciones, posicion);
            }
            Some(Result::Ok(Key::Char(' '))) => {
                selecciones.push(posicion);
                // (*selecciones).append(2);
                return menu_seleccion(mensaje, opciones, selecciones, posicion);
            }
            Some(Result::Ok(Key::Char('\n') | Key::Char(_))) => return,
            Some(_) => {
                println!("Saliendo con el valor de SALIR {}\r", SALIR);
                return;
            }
            None => return,
        }
    }
}

mod opciones_tareas {
    use std::io::stdin;

    use crate::{
        tareas::{Estado, IdTareas, Tarea},
        vista::pausar_programa,
    };

    use super::generar::{self, menu_opciones};

    pub fn opciones_tarea(tarea: &mut Tarea, tareas: &Vec<Tarea>) -> () {
        let datos = vec![
            format!("ID ({})", tarea.id),
            format!("Nombre ({})", tarea.nombre),
            format!("Descripcion ({})", tarea.descripcion),
            format!("Estado ({})", tarea.estado),
            "Cancelar".to_string(),
        ];

        let mut buf: String = String::new();

        match generar::menu_lista(&datos, generar::POSICION_INICIO as usize) {
            1 => loop {
                println!("Introduce el nuevo ID (Antes, {})", tarea.id);

                stdin().read_line(&mut buf).unwrap_or_else(|error| {
                    crate::errores::error_fin(
                        format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                        1,
                    )
                });

                if let Ok(id) = buf.trim().parse::<i32>() {
                    if tareas.id_disponible(id) {
                        tarea.id = id;
                        break;
                    }

                    pausar_programa(format!(
                        "El id {} ya existe (Pulsa para continuar).",
                        buf.trim()
                    ));
                } else {
                    pausar_programa(format!(
                        "El id {} no es valido (Pulsa para continuar).",
                        buf.trim()
                    ));
                }

                buf.clear();
            },
            2 => {
                println!("Introduce el nuevo nombre (Antes, {})", tarea.nombre);
                stdin().read_line(&mut buf).unwrap_or_else(|error| {
                    crate::errores::error_fin(
                        format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                        1,
                    )
                });

                tarea.nombre = buf.trim().to_string();
            }
            3 => {
                println!(
                    "Introduce la nueva descripcion (Antes, {})",
                    tarea.descripcion
                );

                stdin().read_line(&mut buf).unwrap_or_else(|error| {
                    crate::errores::error_fin(
                        format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                        1,
                    );
                });

                tarea.descripcion = buf.trim().to_string();
            }
            4 => {
                let opcion = menu_opciones(
                    format!("Introduce el nuevo estado (Antes, {})", tarea.estado).as_str(),
                    Estado::vec(),
                    generar::POSICION_INICIO as usize,
                );

                if opcion == 0 {
                    tarea.estado = Estado::obtener_estado(opcion);
                } else {
                    tarea.estado = Estado::obtener_estado(opcion - 1);
                }
            }
            _ => return,
        }
    }
}

fn pausar_programa<T: std::fmt::Display>(mensaje: T) {
    println!("{}", mensaje);

    stdo().into_raw_mode().unwrap();

    stdin().keys().next();
}

pub fn mostrar_cursor() -> Result<usize> {
    return stdo().write(format!("{}", cursor::Show).as_bytes());
}

pub fn iniciar_menu() {
    let mut tareas = Vec::<Tarea>::new();

    tareas.push(Tarea {
        id: 1,
        nombre: "Prueba 1".to_string(),
        descripcion: "Pruebas 2".to_string(),
        estado: Estado::FINALIZADO,
    });

    tareas.push(Tarea {
        id: 2,
        nombre: "Prueba 3".to_string(),
        descripcion: "Pruebas 4".to_string(),
        estado: Estado::ENPROCESO,
    });

    tareas.push(Tarea {
        id: 3,
        nombre: "Prueba 5".to_string(),
        descripcion: "Pruebas 6".to_string(),
        estado: Estado::PENDIENTE,
    });

    tareas.push(Tarea {
        id: 4,
        nombre: "Prueba 7".to_string(),
        descripcion: "Pruebas 8".to_string(),
        estado: Estado::PENDIENTE,
    });

    tareas.push(Tarea {
        id: 5,
        nombre: "Prueba 9".to_string(),
        descripcion: "Pruebas 10".to_string(),
        estado: Estado::PENDIENTE,
    });

    tareas.push(Tarea {
        id: 6,
        nombre: "Prueba 11".to_string(),
        descripcion: "Pruebas 12".to_string(),
        estado: Estado::PENDIENTE,
    });

    //----------------------------------------------------------------
    let menu_principal: Vec<&str> = Vec::from([
        "Crear nueva tarea",
        "Seccionar tareas",
        "Buscar tareas",
        "Cerrar",
    ]);

    loop {
        match generar::menu_lista(&menu_principal, generar::POSICION_INICIO as usize) {
            0 => continue,

            1 => crear_tarea(&mut tareas),
            2 => mostrar_tareas(&mut tareas, None),
            3 => buscar_tareas(&mut tareas),

            // 4 => {}
            _ => {
                mostrar_cursor().unwrap_or_else(|error| {
                    print!("Error al mostrar el cursor {}", error);
                    return 1;
                });
                break;
            } // _ => println!("Opcion desconocida o no implementada"),
        }

        pausar_programa("\nPulsa cualquier tecla para continuar");
    }
}

fn crear_tarea(tareas: &mut Vec<Tarea>) -> () {
    let mut buffer = String::new();

    let mut id: i32;
    let mut nombre: String;
    let mut descripcion: String;
    let mut estado: Estado;

    'externo: loop {
        println!("{}{}", clear::All, cursor::Goto(1, 1));
        loop {
            println!(
                "Introduce el ID de la tarea (Pulsa enter para asignarle un valor por defecto)."
            );

            stdin().read_line(&mut buffer).unwrap_or_else(|error| {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            });

            if buffer.trim() == "" {
                id = tareas.siguente_id_disp();
                buffer.clear();
                println!("{}El cursor asignado sera {}", cursor::Up(1), id);
                break;
            } else {
                if let Ok(datos) = buffer.trim().parse::<i32>() {
                    id = datos;
                    buffer.clear();

                    if !tareas.id_disponible(id) {
                        println!();

                        pausar_programa(format!(
                            "El id {} ya existe, selecciona otro. Pulsa una tecla para continuar",
                            id
                        ));
                        println!("{}{}", clear::All, cursor::Goto(1, 1));

                        continue;
                    }

                    break;
                } else if let Err(_) = buffer.parse::<i32>() {
                    pausar_programa(format!("\n{} no es un número valido, introduce uno valido. Pulsa cualquier tecla para continuar", buffer.trim()));
                    buffer.clear();
                }
            }
        }

        println!("\nIntroduce un nombre");
        stdin().read_line(&mut buffer).unwrap_or_else(|error| {
            error_fin(
                format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                1,
            );
        });

        nombre = buffer.trim().to_string();
        buffer.clear();

        println!("\nIntroduce una descripcion");
        stdin().read_line(&mut buffer).unwrap_or_else(|error| {
            error_fin(
                format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                1,
            );
        });

        descripcion = buffer.trim().to_string();
        buffer.clear();

        let estado_opcion = generar::menu_opciones(
            "Selecciona una de las opciones",
            Estado::vec(),
            generar::POSICION_INICIO as usize,
        );

        if estado_opcion == 0 {
            estado = Estado::obtener_estado(estado_opcion);
        } else {
            estado = Estado::obtener_estado(estado_opcion - 1);
        }
        loop {
            print!("{}{}", clear::All, cursor::Goto(1, 1));

            println!(
            "La tarea resultante será:\n\tID -> {}\n\tNombre -> {}\n\tDescripción -> {}\n\tEstado -> {}",
            id, nombre, descripcion, estado
         );

            pausar_programa("\nPulsa cualquier tecla para continuar");

            let opcion = generar::menu_opciones(
                "¿Guardar tarea?",
                vec!["Si", "No", "Cancelar", "Ver Tarea"],
                generar::POSICION_INICIO as usize,
            );
            match opcion {
                1 => break 'externo,
                2 => break,
                0 | 3 => {
                    return ();
                }
                4 => continue,
                _ => break,
            }
        }
    }

    tareas.push(Tarea {
        id,
        nombre,
        descripcion,
        estado,
    });
}

fn mostrar_tareas(tareas: &mut Vec<Tarea>, mostar_tareas: Option<&Vec<Tarea>>) -> () {
    println!("{}{}", cursor::Goto(1, 1), clear::All);

    let fn_mostrar_tareas = |tareas: &Vec<Tarea>| -> Option<i32> {
        let mut tareas_texto: Vec<String> = tareas
            .iter()
            .map(|tarea| {
                return format!(
                    "{} -> {}, {}\r\n\t{}",
                    tarea.id, tarea.nombre, tarea.descripcion, tarea.estado
                );
            })
            .collect();

        tareas_texto.insert(0, "Atras".to_string());

        let mut tarea_seleccionada =
            generar::menu_lista(&tareas_texto, generar::POSICION_INICIO as usize) - 1;

        if tarea_seleccionada == 0 {
            return None;
        };

        tarea_seleccionada -= 1;

        return Some(tareas.get(tarea_seleccionada)?.id);
    };

    let tarea_seleccionada: Option<i32>;

    match mostar_tareas {
        Some(tareas) => tarea_seleccionada = fn_mostrar_tareas(tareas),
        None => tarea_seleccionada = fn_mostrar_tareas(tareas),
    }

    let Some(id_tarea) = tarea_seleccionada else {
         return;
    };

    let opcion = generar::menu_opciones(
        format!("Acciones sobre la tarea {}", id_tarea).as_str(),
        vec!["Editar", "Eliminar", "Cerrar"],
        generar::POSICION_INICIO as usize,
    );

    match opcion {
        1 => {
            let tareas_cln = tareas.clone();

            let Some(tarea) = tareas.into_iter().find(|tarea | tarea.id == id_tarea) else {
                    print!("Error");
                    return;
                };

            opciones_tarea(tarea, &tareas_cln);

            return;
        }
        2 => {
            let opcion = generar::menu_opciones(
                format!("¿Desea eliminar la tarea {}?", id_tarea).as_str(),
                vec!["Si", "No"],
                generar::POSICION_INICIO as usize,
            );

            if opcion == 2 {
                return ();
            }

            tareas.retain(|x| x.id != id_tarea);
            pausar_programa(format!("\nTarea {} eliminada", id_tarea));
            return ();
        }
        _ => {
            return ();
        }
    }
}

fn buscar_tareas(tareas: &mut Vec<Tarea>) -> () {
    if tareas.len() == 0 {
        println!("No existen tareas creadas. Es imposible buscar una tarea");
        return;
    }

    let mut buf: String = String::new();
    // let tareas_cln = tareas.clone();

    match generar::menu_opciones(
        "Selecciona la opcion de busqueda",
        vec!["ID", "Nombre", "Descripcion", "Estado"],
        generar::POSICION_INICIO as usize,
    ) {
        1 => {
            loop {
                println!("Introduce el ID de la tarea");

                stdin().read_line(&mut buf).unwrap_or_else(|error| {
                    error_fin(
                        format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                        1,
                    );
                });

                if let Ok(id) = buf.trim().parse::<i32>() {
                    let tareas_cln = tareas.clone();
                    if let Some(tarea) = tareas.buscar_id(id) {
                        opciones_tareas::opciones_tarea(tarea, &tareas_cln);
                        break;
                    };

                    pausar_programa(format!("No existe ninguna tarea con el id {}. Pulsa cualquier tecla para continuar", id));
                    buf.clear();
                } else {
                    pausar_programa(format!("\n{} no es un número valido, introduce uno valido. Pulsa cualquier tecla para continuar", buf.trim()));
                    buf.clear();
                }
            }
        }
        2 => {
            println!("Introduce el nombre de la tarea");

            stdin().read_line(&mut buf).unwrap_or_else(|error| {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            });

            let tareas_filtradas = tareas.buscar_nombre(buf.trim().to_string());
            mostrar_tareas(tareas, Some(&tareas_filtradas));
        }
        3 => {
            println!("Introduce la descripcion de la tarea");

            stdin().read_line(&mut buf).unwrap_or_else(|error| {
                error_fin(
                    format!("Fallo al ejecutar la aplicacion. Error: {}", error).as_str(),
                    1,
                );
            });

            let tareas_filtradas = tareas.buscar_descripcion(buf.trim().to_string());
            mostrar_tareas(tareas, Some(&tareas_filtradas));
        }
        4 => {
            let mut selecciones = Vec::<usize>::new();

            generar::menu_seleccion(
                "Selecciona la el estado.",
                Estado::vec(),
                &mut selecciones,
                generar::POSICION_INICIO as usize,
            );

            let tareas_filtradas = selecciones
                .into_iter()
                .flat_map(|estado| -> Vec<Tarea> {
                    return tareas.buscar_estado(Estado::obtener_estado(estado));
                })
                .collect::<Vec<Tarea>>();

            // let mut tareas_filtradas = Vec::<&mut Tarea>::new();

            // let estados = selecciones
            //     .iter()
            //     .map(|estado| Estado::obtener_estado(*estado))
            //     .collect::<Vec<Estado>>();

            // let estado = Estado::obtener_estado(se);
            // let mut tareas_filtradas = tareas.buscar_estados(&estados);

            // tareas_cln.buscar_estado(Estado::obtener_estado(_));
            mostrar_tareas(tareas, Some(&tareas_filtradas));
        }
        _ => {
            println!("Error");
        }
    }
}
