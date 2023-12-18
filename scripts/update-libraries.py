import argparse
import dataclasses
import shutil
import subprocess
import tomllib
from pathlib import Path
from typing import Literal, Self

_PROJECT_DIR = (Path(__file__) / "../..").resolve()


@dataclasses.dataclass(slots=True, kw_only=True)
class CargoPackage:
    @dataclasses.dataclass(slots=True, kw_only=True)
    class BuilderMeta:
        ignore: bool = False
        repo: str | None = None

    cargo_path: Path
    name: str
    version: str
    links: str | None
    builder_meta: BuilderMeta

    @classmethod
    def load(cls, cargo_path: Path) -> Self:
        with cargo_path.open("rb") as fp:
            metadata = tomllib.load(fp)

        package = metadata["package"]
        builder_meta = cls.BuilderMeta()

        try:
            builder_meta_data = package["metadata"]["aws-c-builder"]
        except KeyError:
            pass
        else:
            builder_meta.ignore = bool(builder_meta_data.get("ignore", False))
            builder_meta.repo = builder_meta_data.get("repo")

        return cls(
            cargo_path=cargo_path,
            name=package["name"],
            version=package["version"],
            links=package.get("links"),
            builder_meta=builder_meta,
        )

    def get_repo_url(self) -> str:
        return self.builder_meta.repo or f"https://github.com/awslabs/{self.links}.git"

    def get_repo_tag(self) -> str:
        _, _, repo_tag = self.version.partition("+")
        assert repo_tag, "missing repo tag"
        return repo_tag


def _apply_package_code(package: CargoPackage) -> None:
    assert package.links

    repo_tag = package.get_repo_tag()
    lib_dir = package.cargo_path.parent / package.links
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
            package.get_repo_url(),
            lib_dir,
        ],
        check=True,
        cwd=_PROJECT_DIR,
    )
    shutil.rmtree(lib_dir / ".git")


def _check_package_update(package: CargoPackage) -> None:
    current_tag = package.get_repo_tag()
    tags = _list_version_tags(package.get_repo_url())
    newest_tag = tags[0]

    if current_tag != newest_tag:
        print(f"Package {package.name} can be updated to {newest_tag}")


def _list_version_tags(repo_url: str) -> list[str]:
    proc = subprocess.run(
        ["git", "ls-remote", "--tags", "--refs", "--sort=-v:refname", repo_url],
        stdout=subprocess.PIPE,
        check=True,
        cwd=_PROJECT_DIR,
        encoding="utf-8",
    )
    tags: list[str] = []
    for line in proc.stdout.splitlines():
        _oid, _, ref = line.partition("\t")
        assert ref.startswith("refs/tags/")
        tags.append(ref[len("refs/tags/") :])

    return tags


def _arg_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(required=True)

    parser_apply = subparsers.add_parser("apply")
    parser_apply.set_defaults(op="apply")

    parser_check = subparsers.add_parser("check")
    parser_check.set_defaults(op="check")

    return parser


def _parse_args() -> argparse.Namespace:
    parser = _arg_parser()
    return parser.parse_args()


def main() -> None:
    ns = _parse_args()
    op: Literal["apply"] | Literal["check"] = ns.op

    for package_path in (_PROJECT_DIR / "packages").iterdir():
        if not package_path.is_dir():
            continue

        try:
            package = CargoPackage.load(package_path / "Cargo.toml")
            if package.builder_meta.ignore:
                continue

            if op == "apply":
                _apply_package_code(package)
            else:
                _check_package_update(package)
        except Exception:
            print(f"Error in package {package_path.name}")
            raise


if __name__ == "__main__":
    main()
