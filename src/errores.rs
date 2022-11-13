use crate::vista::mostrar_cursor;

pub fn error_fin(msg: &str, codigo: i32) -> ! {
    mostrar_cursor().unwrap_or_else(|error| {
        println!("Fallo al mostrar el cursor. Error {}", error);
        return 1;
    });
    println!("{}", msg);
    std::process::exit(codigo);
}
