use std::path::{Path, PathBuf};

pub fn run(lib_dir: &Path, include_dirs: &[PathBuf], source_subdirs: &[&str]) {
    println!("cargo:rerun-if-changed={}", lib_dir.to_str().unwrap());

    let lib_name = lib_dir.file_name().unwrap().to_str().unwrap();
    let mut build = cc::Build::new();
    build
        .warnings(true)
        .extra_warnings(true)
        .includes(include_dirs);

    let source_dir = lib_dir.join("source");
    build_files_dir(&mut build, &source_dir);
    for subdir in source_subdirs {
        build_files_dir(&mut build, &source_dir.join(subdir));
    }

    build.compile(lib_name);
}

fn build_files_dir(build: &mut cc::Build, path: &Path) {
    for item in path.read_dir().expect("read dir") {
        let item = item.unwrap();
        let file_type = item.file_type().unwrap();
        if !file_type.is_file() {
            continue;
        }
        let file_name = item.file_name();
        let name = file_name.to_str().unwrap();
        if !name.ends_with(".c") {
            continue;
        }
        build.file(item.path());
    }
}
