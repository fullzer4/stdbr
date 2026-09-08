from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


kind, artifact_directory, *rest = sys.argv[1:]
expected_version = rest[0] if rest else ""
artifacts = list(Path(artifact_directory).glob("*.whl" if kind == "wheel" else "*.tar.gz"))
if len(artifacts) != 1:
    raise SystemExit(f"expected exactly one {kind} artifact, found {len(artifacts)}")

temporary_directory = Path(tempfile.mkdtemp(prefix="stdbr-python-smoke-"))
try:
    environment = os.environ.copy()
    environment.pop("PYTHONPATH", None)
    virtualenv = temporary_directory / "venv"
    subprocess.run([sys.executable, "-m", "venv", str(virtualenv)], check=True, env=environment)
    python = virtualenv / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
    subprocess.run(
        [str(python), "-m", "pip", "install", "--no-cache-dir", str(artifacts[0].resolve())],
        check=True,
        cwd=temporary_directory,
        env=environment,
    )
    check = (
        "import importlib.metadata, pathlib, stdbr; "
        "assert stdbr.cpf_is_valid('52998224725'); "
        "assert any(pathlib.Path(p).name == 'LICENSE' for p in importlib.metadata.files('stdbr')); "
        f"assert not {expected_version!r} or importlib.metadata.version('stdbr') == {expected_version!r}"
    )
    subprocess.run([str(python), "-c", check], check=True, cwd=temporary_directory, env=environment)
finally:
    shutil.rmtree(temporary_directory, ignore_errors=True)
