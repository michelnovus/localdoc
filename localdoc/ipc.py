# [GLP3] Copyright (C) 2024  Michel Novus

import os
import socket
import json
from typing import Any


def serialize(data: dict[str, Any]) -> bytes:
    """Serializa un 'dict' a un formato JSON binario."""
    if type(data) != type(dict()):
        raise TypeError("se esperaba un elemento tipo dict()")
    json_data = json.dumps(data, separators=(",", ":"))
    return json_data.encode("utf-8")


def deserialize(data: bytes) -> dict[str, Any]:
    """Deserializa una cadena JSON binaria a un 'dict'."""
    json_data = json.loads(data)
    if type(json_data) != type(dict()):
        raise TypeError("se esperaba un elemento tipo dict()")
    return json_data


def _mksocket(timeout: float = 1.0) -> socket.socket:
    """Crea un socket AF_UNIX."""
    unix_socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    unix_socket.settimeout(timeout)
    return unix_socket


def communicate(
    ipc_socket: str,
    data: dict[str, Any],
    timeout: float = 1.0,
    max_buffer_size: int = 65536,
) -> dict[str, Any]:
    """Envía el contenido de 'data' al 'ipc_socket' y espera su respuesta."""

    serialized_data = serialize(data)
    if len(serialized_data) > max_buffer_size:
        raise OverflowError(
            f"el tamaño del mensaje es superior a {max_buffer_size} bytes"
        )
    unix_socket = _mksocket(timeout)
    unix_socket.connect(ipc_socket)
    amount_sent = unix_socket.send(serialized_data)

    response = unix_socket.recv(max_buffer_size)
    deserialize_response = deserialize(response)
    return deserialize_response


class IpcServer(object):
    def __init__(
        self,
        ipc_socket: str,
        timeout: float = 1.0,
        max_buffer_size: int = 65536,
    ) -> None:
        self.max_buffer_size = max_buffer_size
        self.ipc_socket = ipc_socket
        self._socket = _mksocket(timeout)
        self._socket.bind(self.ipc_socket)

        self._connection = None

    def wait_connection(self) -> dict[str, Any]:
        """Espera por conexiones de clientes."""

        self._conn = self._socket.accept()[0]
        data = self._conn.recv(self.max_buffer_size)
        data = deserialize(data)
        return data

    def reply(self, data: dict[str, Any]) -> None:
        """Responde a la comunicación en curso y cierra el socket."""
        serialized_data = serialize(data)
        if len(serialized_data) > self.max_buffer_size:
            raise OverflowError(
                f"el tamaño del mensaje es superior a {self.max_buffer_size} bytes"
            )
        amount_sent = self._conn.send(serialized_data)

        self._conn.shutdown(socket.SHUT_RD)
        self._conn.close()

    def close(self) -> None:
        """Cierra el socket."""
        self._socket.shutdown(socket.SHUT_RD)
        self._socket.close()
        os.unlink(self.ipc_socket)
