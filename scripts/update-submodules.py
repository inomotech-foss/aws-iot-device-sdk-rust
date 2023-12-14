import shutil
import subprocess
import tomllib
from pathlib import Path
from typing import TypedDict, cast

_PROJECT_DIR = (Path(__file__) / "../..").resolve()


class Package(TypedDict, total=False):
    name: str
    version: str
    links: str


class CargoMetadata(TypedDict, total=False):
    package: Package


def _update_package_submodule(cargo_path: Path) -> None:
    with cargo_path.open("rb") as fp:
        metadata = cast(CargoMetadata, tomllib.load(fp))

    try:
        c_lib_name = metadata["package"]["links"]
        version = metadata["package"]["version"]
    except KeyError:
        raise

    _, _, repo_tag = version.partition("+")
    assert repo_tag

    lib_dir = cargo_path.parent / c_lib_name
    if lib_dir.exists():
        shutil.rmtree(lib_dir)

    repo_url = f"https://github.com/awslabs/{c_lib_name}"
    subprocess.run(
        [
            "git",
            "clone",
            f"--branch={repo_tag}",
            "--depth=1",
            "--",
            repo_url,
            lib_dir,
        ],
        check=True,
        cwd=_PROJECT_DIR,
    )


def main() -> None:
    for package_path in (_PROJECT_DIR / "packages").iterdir():
        if not package_path.is_dir():
            continue
        _update_package_submodule(package_path / "Cargo.toml")


if __name__ == "__main__":
    main()
