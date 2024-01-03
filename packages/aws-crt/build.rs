fn main() {
    let root_paths = aws_c_builder::get_dependency_root_paths(["AWS_C_COMMON"]);
    let dependency_includes = root_paths.into_iter().map(|path| format!("{path}/include"));

    println!("cargo:rerun-if-changed=src/glue/logging.c");
    cc::Build::new()
        .warnings(true)
        .extra_warnings(true)
        .includes(dependency_includes)
        .file("src/glue/logging.c")
        .compile("glue");
}
