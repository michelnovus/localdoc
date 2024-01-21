# [MIT License] Copyright (c) 2024 Michel Novus

from threading import Lock
from socketserver import BaseRequestHandler


class DocProcess(object):
    """"""

    def __init__(
        self, documentation_name: str, *popen_args, **popen_kwargs
    ) -> None:
        self.documentation_name: str = documentation_name
        self._popen_args = popen_args
        self._popen_kwargs = popen_kwargs

    def start(self) -> None:
        pass

    def stop(self, kill: bool = False) -> None:
        pass


class ServedDocs(object):
    """Contiene y modifica los estados de los procesos servidos."""

    def __init__(self) -> None:
        self._lock = Lock()
        self._subprocess: dict[str, DocProcess] = {}

    def add_process(self, process: DocProcess) -> None:
        """Añade un nuevo proceso a la memoria."""
        with self._lock:
            self._subprocess[process.documentation_name] = process

    def start_process(self, documentation_name: str) -> None:
        """Inicia el proceso elegido."""
        with self._lock:
            self._subprocess[documentation_name].start()

    def stop_process(self, documentation_name: str) -> None:
        """Detiene y elimina el proceso con el nombre de 'documentation_name'."""
        with self._lock:
            self._subprocess[documentation_name].stop()
            self._subprocess.pop(documentation_name)

    def get_doc_names(self) -> list[str]:
        """Retorna las documentaciones actualmente servidas."""
        with self._lock:
            served_documentation = list(self._subprocess.keys())
        return served_documentation


class DispatcherHandler(BaseRequestHandler):
    def handle(self) -> None:
        pass
