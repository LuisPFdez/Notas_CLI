use crate::vista::mostrar_cursor;

pub fn error_fin(msg: &str, codigo: i32) -> ! {
    println!("{}", msg);
    mostrar_cursor().expect("Error al mostrar el cursor");
    std::process::exit(codigo);
}

pub fn error_mostrar (msg: impl std::error::Error ) -> () {
    print!("{}", msg);
    std::process::exit(1);
}