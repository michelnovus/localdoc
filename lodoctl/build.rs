fn main() {
    set_profile_env_to_rustc();
}

/// Define cual es la ruta final al binario de `lodosrv` al compilar.
/// Si se compila con el perfil `debug` la ruta a lodosrv es en target
/// (cargo run / cargo build), si se utiliza el prefil `release` la
/// ruta a lodosrv será definida en el sistema de archivos.
/// La variable de entorno `PROFILE_TYPE` debe ser capturada en
/// tiempo de compilación con la macro `env!` y ser discriminada
/// entre los valores `debug` y `release`.
///
/// Esto existe porque el programa `lodosrv` esta destinado a ser
/// lanzado (mediate fork) por `lodoctl start` y en en desarrollo
/// el binario de `lodosrv` está en `target/debug` y en producción
/// en el sistema de archivos, sea `/usr/lib` o `~/.local/lib`.
fn set_profile_env_to_rustc() {
    let profile = std::env::var("PROFILE").unwrap();
    if &profile == &"debug".to_string() {
        println!("cargo:rustc-env=PROFILE_TYPE=debug");
    } else if &profile == &"release".to_string() {
        println!("cargo:rustc-env=PROFILE_TYPE=release");
    };
}
