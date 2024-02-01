// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;
use localdoc_service::resolve_socket_filepath;
use std::process;

fn main() {
    let socket_file =
        resolve_socket_filepath("LOCALDOC_SOCKET").unwrap_or_else(|| process::exit(1));
    println!("{socket_file}");
}
