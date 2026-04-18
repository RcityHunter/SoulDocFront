from __future__ import annotations

import argparse
import mimetypes
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlparse


class SpaRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, directory: str, **kwargs):
        self.root = Path(directory).resolve()
        super().__init__(*args, directory=directory, **kwargs)

    def translate_path(self, path: str) -> str:
        parsed = urlparse(path)
        clean_path = unquote(parsed.path.lstrip("/"))
        candidate = (self.root / clean_path).resolve()

        if self._is_safe(candidate):
            if candidate.is_file():
                return str(candidate)
            if candidate.is_dir():
                index_file = candidate / "index.html"
                if index_file.is_file():
                    return str(index_file)

        return str(self.root / "index.html")

    def guess_type(self, path: str) -> str:
        mime_type, _ = mimetypes.guess_type(path)
        return mime_type or "application/octet-stream"

    def log_message(self, format: str, *args) -> None:
        message = "%s - - [%s] %s\n" % (self.address_string(), self.log_date_time_string(), format % args)
        print(message, end="")

    def _is_safe(self, candidate: Path) -> bool:
        try:
            candidate.relative_to(self.root)
            return True
        except ValueError:
            return False


def main() -> None:
    parser = argparse.ArgumentParser(description="Serve static files with SPA fallback.")
    parser.add_argument("--root", required=True, help="Directory to serve")
    parser.add_argument("--host", default="127.0.0.1", help="Host to bind")
    parser.add_argument("--port", type=int, default=8080, help="Port to bind")
    args = parser.parse_args()

    directory = str(Path(args.root).resolve())

    def handler(*handler_args, **handler_kwargs):
        return SpaRequestHandler(*handler_args, directory=directory, **handler_kwargs)

    server = ThreadingHTTPServer((args.host, args.port), handler)
    print(f"Serving {directory} at http://{args.host}:{args.port}")
    server.serve_forever()


if __name__ == "__main__":
    main()
