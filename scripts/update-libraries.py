import shutil
import subprocess
import tomllib
from pathlib import Path
from typing import Any, TypedDict, cast

_PROJECT_DIR = (Path(__file__) / "../..").resolve()


class CargoPackage(TypedDict, total=False):
    name: str
    version: str
    links: str
    metadata: dict[str, Any]


class CargoRoot(TypedDict, total=False):
    package: CargoPackage


class BuilderMeta(TypedDict, total=False):
    ignore: bool
    repo: str


def _update_package_submodule(cargo_path: Path) -> None:
    with cargo_path.open("rb") as fp:
        metadata = cast(CargoRoot, tomllib.load(fp))

    try:
        builder_meta: BuilderMeta = metadata["package"]["metadata"]["aws-c-builder"]
    except KeyError:
        builder_meta: BuilderMeta = {}

    if builder_meta.get("ignore"):
        return

    try:
        c_lib_name = metadata["package"]["links"]
        version = metadata["package"]["version"]
    except KeyError:
        raise

    _, _, repo_tag = version.partition("+")
    assert repo_tag

    try:
        repo_url = builder_meta["repo"]
    except KeyError:
        repo_url = f"https://github.com/awslabs/{c_lib_name}.git"

    lib_dir = cargo_path.parent / c_lib_name
    if lib_dir.exists():
        shutil.rmtree(lib_dir)

    subprocess.run(
        [
            "git",
            "-c",
            "advice.detachedHead=false",
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
    shutil.rmtree(lib_dir / ".git")


def main() -> None:
    for package_path in (_PROJECT_DIR / "packages").iterdir():
        if not package_path.is_dir():
            continue
        try:
            _update_package_submodule(package_path / "Cargo.toml")
        except Exception:
            print(f"Error in package {package_path.name}")
            raise


if __name__ == "__main__":
    main()
