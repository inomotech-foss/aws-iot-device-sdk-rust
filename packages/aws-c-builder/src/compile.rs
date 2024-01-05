use std::borrow::Cow;
use std::path::Path;

use crate::Builder;

pub fn run(builder: &mut Builder, include_dirs: &[Cow<Path>]) {
    println!(
        "cargo:rerun-if-changed={}",
        builder.lib_dir.to_str().unwrap()
    );

    let lib_name = builder.lib_dir.file_name().unwrap().to_str().unwrap();
    let mut build = cc::Build::new();
    build
        .warnings(true)
        .extra_warnings(true)
        .includes(include_dirs)
        .define("INTEL_NO_ITTNOTIFY_API", None);

    let source_dir = builder.lib_dir.join("source");
    build_files_dir(&mut build, &source_dir);
    for subdir in &builder.source_subdirs {
        build_files_dir(&mut build, &source_dir.join(subdir));
    }

    for cb in &mut builder.cc_callbacks {
        cb(&mut build);
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
        let is_c_file = item
            .path()
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("c"));
        if !is_c_file {
            continue;
        }
        build.file(item.path());
    }
}
