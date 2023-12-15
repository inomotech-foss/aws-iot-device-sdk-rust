pub fn build(lib_name: &str, deps: &[&str]) {
    let cmake_prefix_path = get_cmake_prefix_path(deps).join(";");
    println!("cargo:cmake_prefix_path={cmake_prefix_path}");

    println!("cargo:rerun-if-changed={lib_name}");
    let out_dir = cmake::Config::new(lib_name)
        .define("CMAKE_PREFIX_PATH", cmake_prefix_path)
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_TESTING", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();
    println!("cargo:rustc-link-search=native={out_dir}/lib");
    println!("cargo:rustc-link-lib=static={lib_name}");
}

fn get_cmake_prefix_path(deps: &[&str]) -> Vec<String> {
    let mut all_paths = Vec::<String>::with_capacity(deps.len());
    for &dep in deps {
        let root = get_build_variable(dep, "ROOT");
        if all_paths.iter().any(|existing| existing == &root) {
            // since it's transitive, we know that we have all its dependencies as well.
            continue;
        }

        all_paths.push(root);
        all_paths.extend(
            get_build_variable(dep, "CMAKE_PREFIX_PATH")
                .split(';')
                .filter(|s| !s.is_empty())
                .map(str::to_owned),
        );
    }

    all_paths.sort_unstable();
    all_paths.dedup();
    all_paths
}

fn get_build_variable(package: &str, var: &str) -> String {
    let Ok(v) = std::env::var(format!("DEP_{package}_{var}")) else {
        panic!("package '{package}' didn't set the '{var}' variable in its build script");
    };
    v
}
