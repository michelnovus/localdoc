//! Testea la comunicación entre el cliente y el servicio.

extern crate localdoc_service;

use localdoc_service::service::{process::RuntimeDir, Service};
use std::{
    io::prelude::*, net::Shutdown, os::unix::net::UnixStream, thread, time,
};

const ROOT_DIR: &'static str = "/tmp/socket_communication_test";
const SOCK_NAME: &'static str = "socket";
const SOCKET: &'static str = "/tmp/socket_communication_test/socket";

#[test]
fn socket_communication_test() {
    let runtime_dir = RuntimeDir::new(&ROOT_DIR, &SOCK_NAME);
    runtime_dir.make().expect("Error al crear los directorios.");
    let mut service = Service::new(runtime_dir);
    thread::spawn(move || match service.start() {
        Ok(()) => println!("Servicio terminado!"),
        Err(_) => println!("Algo salió mal..."),
    });
    thread::sleep(time::Duration::from_secs_f64(0.5));

    {
        let mut stream = UnixStream::connect(&SOCKET).unwrap();
        stream.write_all(r#""EXIT""#.as_bytes()).unwrap();
        stream.shutdown(Shutdown::Write).unwrap();
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        stream.shutdown(Shutdown::Read).unwrap();
        println!("{}", buffer);
    }
}
