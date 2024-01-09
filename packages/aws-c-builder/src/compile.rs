use std::borrow::Cow;
use std::path::Path;

use crate::Builder;

pub fn run(builder: &mut Builder, include_dirs: &[Cow<Path>], enable_tracing: bool) {
    println!(
        "cargo:rerun-if-changed={}",
        builder.lib_dir.to_str().unwrap()
    );

    let lib_name = builder.lib_dir.file_name().unwrap().to_str().unwrap();
    let mut build = cc::Build::new();
    build
        .warnings(true)
        .extra_warnings(false)
        .includes(include_dirs);

    if !enable_tracing {
        build.define("INTEL_NO_ITTNOTIFY_API", None);
    }

    let source_dir = builder.lib_dir.join("source");
    build_add_source(&mut build, &source_dir);
    for path in &builder.source_paths {
        build_add_source(&mut build, &source_dir.join(path));
    }

    for cb in &mut builder.cc_callbacks {
        cb(&mut build);
    }

    build.compile(lib_name);
}

fn build_add_source(build: &mut cc::Build, path: &Path) {
    if path.is_file() {
        build.file(path);
        return;
    }

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
