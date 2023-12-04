# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path
import socket
import json
from enum import Enum

IPCType = Enum("ICPType", ["CLIENT", "SERVER"])


class IPC(object):
    """Permite recibir y enviar datos entre dos procesos independientes
    mediante sockets UNIX.

    El IPC tipo CLIENT implementa el método communicate(), envía un dict
    al IPC tipo SERVER y espera su respuesta, otro dict.

    El IPC tipo SERVER implementa los métodos wait() y reply(). Utiliza el
    método wait() para esperar conexiones de otros procesos y utiliza después
    reply() para responder y terminar la conexion.
    """

    def __init__(
        self, socket_path: str, type: IPCType, timeout: float = 1.0
    ) -> None:
        self.type = type
        self.socket_path = socket_path
        self.timeout = timeout
        self.buffer_size = 65536  # 64KiB

        self._connection: socket.socket = None  # type: ignore

        if self.type == IPCType.SERVER:
            if os.path.exists(self.socket_path):
                raise FileExistsError(f"Ya existe el socket {self.socket_path}")
            self.unix_socket = self._mksocket()
            self.unix_socket.bind(self.socket_path)
            self.unix_socket.listen(1)
        elif self.type == IPCType.CLIENT:
            if not os.path.exists(self.socket_path):
                raise FileNotFoundError("No existe el socket")

        if not (self.type == IPCType.CLIENT or self.type == IPCType.SERVER):
            raise ValueError(f"type: {self.type} no es válido!")

    def communicate(self, data: dict[str, str]) -> dict[str, str]:
        """Envía un 'dict' al otro proceso y espera su respuesta como 'dict'.
        Solo disponible en IPC typo cliente."""
        self._raise_type_error(target_type=IPCType.CLIENT)
        serialized_data = self._serialize(data)
        if len(serialized_data) > self.buffer_size:
            raise OverflowError(
                f"se esperaba 'data' <= {self.buffer_size} bytes, se tiene "
                f"'data' == {len(serialized_data)} bytes."
            )
        unix_socket = self._mksocket()
        unix_socket.connect(self.socket_path)
        unix_socket.sendall(serialized_data)
        response = self._deserialize(unix_socket.recv(self.buffer_size))
        return response

    def wait(self) -> dict[str, str]:
        """Espera una solicitud de un proceso ajeno. Llamada bloqueante.

        Solo disponible en IPC typo servidor.

        Debe estar seguido del método reply().
        """
        self._raise_type_error(target_type=IPCType.SERVER)
        self._connection = self.unix_socket.accept()[0]
        data = self._deserialize(self._connection.recv(self.buffer_size))
        return data

    def reply(self, data: dict[str, str]) -> None:
        """Mensaje de respuesta del servidor al cliente, se debe usar después
        del método wait()."""
        self._raise_type_error(target_type=IPCType.SERVER)
        serialized_data = self._serialize(data)
        if len(serialized_data) > self.buffer_size:
            raise OverflowError(
                f"se esperaba 'data' <= {self.buffer_size} bytes, se tiene "
                f"'data' == {len(serialized_data)} bytes."
            )
        self._connection.sendall(serialized_data)
        self._connection.shutdown(socket.SHUT_RD)
        self._connection.close()

    def close(self) -> None:
        """Cierra el socket."""
        self.unix_socket.shutdown(socket.SHUT_RD)
        self.unix_socket.close()
        os.unlink(self.socket_path)

    def get_data_size(self, data: dict[str, str]) -> int:
        """Devuelve el tamaño de los datos serializados que se enviarán."""
        return len(self._serialize(data))

    def _mksocket(self) -> socket.socket:
        """Crea un nuevo socket UNIX."""
        return socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

    def _serialize(self, data: dict[str, str]) -> bytes:
        json_data = json.dumps(data, separators=(",", ":"))
        return json_data.encode("utf-8")

    def _deserialize(self, data: bytes) -> dict[str, str]:
        json_data = json.loads(data)
        return json_data

    def _raise_type_error(self, target_type: IPCType) -> None:
        """Eleva la excepción NotImplementedError para el tipo de IPC."""

        if target_type == IPCType.SERVER and self.type != IPCType.SERVER:
            raise NotImplementedError("Solo disponible en IPC tipo SERVER")
        elif target_type == IPCType.CLIENT and self.type != IPCType.CLIENT:
            raise NotImplementedError("Solo disponible en IPC tipo CLIENT")
