"""Build and validate the package from the installed-consumer seam."""

from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import zipfile


ROOT = Path(__file__).resolve().parent


def run(*args: str, cwd: Path = ROOT, env: dict[str, str] | None = None) -> None:
    print("+", *args, flush=True)
    subprocess.run(args, cwd=cwd, env=env, check=True)


def main() -> None:
    dist = ROOT / "dist"
    shutil.rmtree(dist, ignore_errors=True)
    run(sys.executable, "-m", "build", "--no-isolation", "--wheel", "--sdist")
    wheel = next(dist.glob("*.whl"))
    if not next(dist.glob("*.tar.gz"), None):
        raise RuntimeError("sdist was not produced")
    with zipfile.ZipFile(wheel) as archive:
        names = set(archive.namelist())
        for required in (
            "cameronapak_platform_sdk/py.typed",
            "cameronapak_platform_sdk/sdk-map.json",
            "cameronapak_platform_sdk/client.py",
        ):
            if required not in names:
                raise RuntimeError(f"wheel is missing {required}")

    with tempfile.TemporaryDirectory(prefix="platform-sdk-consumer-") as directory:
        consumer = Path(directory)
        source = consumer / "source"
        with tarfile.open(next(dist.glob("*.tar.gz"))) as archive:
            archive.extractall(source, filter="data")
        rebuilt = consumer / "rebuilt"
        run(sys.executable, "-m", "build", "--no-isolation", "--wheel", "--outdir", str(rebuilt), str(next(source.iterdir())), cwd=consumer)
        with zipfile.ZipFile(wheel) as first, zipfile.ZipFile(next(rebuilt.glob("*.whl"))) as second:
            if {name: first.read(name) for name in first.namelist()} != {name: second.read(name) for name in second.namelist()}:
                raise RuntimeError("wheel contents differ when rebuilt from the sdist")
        print("Wheel contents reproduce from the sdist", flush=True)
        requirements = consumer / "requirements.txt"
        run("uv", "export", "--frozen", "--no-emit-project", "--no-hashes", "--output-file", str(requirements))
        venv = consumer / ".venv"
        run("uv", "venv", "--python", sys.executable, str(venv), cwd=consumer)
        python = venv / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
        run("uv", "pip", "install", "--python", str(python), "--constraint", str(requirements), str(wheel), cwd=consumer)
        environment = os.environ.copy()
        environment.pop("PYTHONPATH", None)
        environment["PYTHONNOUSERSITE"] = "1"
        environment["SDK_REPOSITORY"] = str(ROOT.parents[1])
        run(str(python), "-m", "unittest", "discover", "-s", str(ROOT / "test"), "-v", cwd=consumer, env=environment)
        run("uv", "pip", "install", "--python", str(python), "--constraint", str(requirements), "mypy", cwd=consumer)
        run(str(python), "-m", "mypy", "--strict", "--warn-unused-ignores", str(ROOT / "test" / "typing_consumer.py"), cwd=consumer, env=environment)


if __name__ == "__main__":
    main()
