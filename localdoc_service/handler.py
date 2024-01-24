# [MIT License] Copyright (c) 2024 Michel Novus

import json
from socketserver import BaseRequestHandler


class HeaderInfo(object):
    """Header of connection communication in JSON format:
    ["app_name","message_hexsize","MD5_hexdigest"].

    Provides application name (app_id), the total size in bytes of
    stream (size) and hashsum of total JSON string (hashsum).
    """

    def __init__(self, data: bytes) -> None:
        try:
            self._data = json.loads(data.decode())
            self.app_id = self._data[0]
            self.size = int(self._data[1], 16)
            self.hashsum = self._data[2]
        except (json.JSONDecodeError, ValueError, IndexError) as e:
            raise TypeError(
                "data is not valid header syntax, it's expect: "
                '["app_name","message_hexsize","MD5_hexdigest"]'
            )


class ConnectionHandler(BaseRequestHandler):
    def handle(self) -> None:
        data = self.request.recv(64)
        header = HeaderInfo(data)

        self.request.sendall("ok".encode())
