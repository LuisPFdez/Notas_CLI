mod errores;
mod tareas;
mod vista;

use vista::iniciar_menu;

fn main() -> Result<(), ()> {
    iniciar_menu();

    return Ok(());
}
