//! Testea la comunicación entre el cliente y el servicio.

extern crate localdoc_service;

use localdoc_service::service::Service;
use std::{os::unix::net::UnixStream, thread};

const SOCKET: &'static str = "/tmp/socket_communication_test";

#[test]
fn socket_communication_test() {}
