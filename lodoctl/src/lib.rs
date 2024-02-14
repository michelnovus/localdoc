// [MIT License] Copyright (c) 2024 Michel Novus

/// Módulo de funciones que imprimen mensajes en la terminal.
pub mod print {
    use std::path::PathBuf;

    /// Imprime una línea con el nombre del programa y su versión.
    pub fn version(app_path: &PathBuf, version: &str) {
        println!(
            "{} {}",
            app_path.file_name().unwrap().to_string_lossy(),
            version
        );
    }

    /// Imprime el típico mensaje de ayuda.
    pub fn help(app_path: &PathBuf) {
        println!(
            "`lodoctl` brinda una interfaz en consola para
            \rla comunicación con el servicio `lodosrv`."
        );
        println!();
        println!(
            "uso: {} {{comando}} [argumento, ...]",
            app_path.file_name().unwrap().to_string_lossy()
        );
        println!();
        println!("Comandos:");
        println!("    --help            Imprime esta ayuda");
        println!("    --version         Muestra la versión actual");
    }

    /// Imprime una línea que avisa de un comando inválido.
    pub fn unknown_cmd(cmd: &String) {
        println!("No se reconoce el comando: `{cmd}`");
    }
}
