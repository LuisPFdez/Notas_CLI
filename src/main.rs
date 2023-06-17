mod errores;
mod tareas;
mod vista;

use vista::iniciar_menu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    iniciar_menu()?;

    return Ok(());
}
