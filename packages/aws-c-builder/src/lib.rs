//! Build script dependency for all related aws c library packages.

use std::path::PathBuf;

pub struct Config {
    lib_name: String,
    aws_dependencies: Vec<String>,
    link_libraries: Vec<String>,
    include_dir_names: Vec<String>,
    cmake_callback: Option<Box<dyn FnOnce(&mut cmake::Config)>>,
    run_bindgen: bool,
    bindgen_callback: Option<Box<dyn FnOnce(bindgen::Builder) -> bindgen::Builder>>,
}

impl Config {
    pub fn new(lib_name: impl Into<String>) -> Self {
        let lib_name = lib_name.into();
        let link_libraries = vec![lib_name.clone()];
        Self {
            lib_name,
            aws_dependencies: Vec::new(),
            link_libraries,
            include_dir_names: Vec::new(),
            cmake_callback: None,
            run_bindgen: true,
            bindgen_callback: None,
        }
    }

    pub fn aws_dependencies<I>(&mut self, deps: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        self.aws_dependencies = deps.into_iter().map(|s| s.as_ref().to_owned()).collect();
        self
    }

    pub fn link_libraries<I>(&mut self, libs: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        self.link_libraries = libs.into_iter().map(|s| s.as_ref().to_owned()).collect();
        self
    }

    pub fn include_dir_names<I>(&mut self, names: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        self.include_dir_names = names.into_iter().map(|s| s.as_ref().to_owned()).collect();
        self
    }

    pub fn cmake_callback(
        &mut self,
        callback: impl FnOnce(&mut cmake::Config) + 'static,
    ) -> &mut Self {
        self.cmake_callback = Some(Box::new(callback));
        self
    }

    pub fn run_bindgen(&mut self, doit: bool) -> &mut Self {
        self.run_bindgen = doit;
        self
    }

    pub fn bindgen_callback(
        &mut self,
        callback: impl FnOnce(bindgen::Builder) -> bindgen::Builder + 'static,
    ) -> &mut Self {
        self.bindgen_callback = Some(Box::new(callback));
        self
    }

    pub fn build(&mut self) {
        let dependency_root_paths = get_dependency_root_paths(&self.aws_dependencies);
        let cmake_prefix_path = dependency_root_paths.join(";");
        println!("cargo:cmake_prefix_path={cmake_prefix_path}");
        let out_dir = self.compile(&cmake_prefix_path);
        if self.run_bindgen {
            self.generate_bindings(&out_dir, &dependency_root_paths);
        }
    }

    fn compile(&mut self, cmake_prefix_path: &str) -> String {
        println!("cargo:rerun-if-changed={}", self.lib_name);
        let mut config = cmake::Config::new(&self.lib_name);
        config
            .define("CMAKE_PREFIX_PATH", cmake_prefix_path)
            .define("AWS_ENABLE_LTO", "ON")
            .define("BUILD_DEPS", "OFF")
            .define("BUILD_TESTING", "OFF");

        if let Some(cb) = self.cmake_callback.take() {
            cb(&mut config);
        }

        let out_dir = config.build().to_str().unwrap().to_owned();
        println!("cargo:rustc-link-search=native={out_dir}/lib");
        for name in &self.link_libraries {
            println!("cargo:rustc-link-lib=static={name}");
        }
        out_dir
    }

    fn generate_bindings(&mut self, lib_root: &str, dependency_root_paths: &[String]) {
        let include_args = std::iter::once(lib_root)
            .chain(dependency_root_paths.iter().map(String::as_str))
            .map(|path| format!("-I{path}/include"));

        println!("cargo:rerun-if-changed=wrapper.h");
        let mut builder = bindgen::builder()
            .allowlist_recursively(false)
            .array_pointers_in_arguments(true)
            .enable_function_attribute_detection()
            .generate_cstr(true)
            .merge_extern_blocks(true)
            .prepend_enum_name(false)
            .sort_semantically(true)
            .use_core()
            .clang_args(include_args)
            .header("wrapper.h");
        if let Some(cb) = self.bindgen_callback.take() {
            builder = cb(builder);
        }
        for name in &self.include_dir_names {
            builder = builder.allowlist_file(format!(".*/{name}/[^/]+\\.h"));
        }

        let bindings = builder.generate().unwrap();

        let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .unwrap();
    }
}

fn get_dependency_root_paths(deps: &[String]) -> Vec<String> {
    let mut all_paths = Vec::<String>::with_capacity(deps.len());
    for dep in deps {
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
        panic!("package '{package}' didn't set the '{var}' variable in its build script or isn't a direct dependency of this package");
    };
    v
}

pub fn is_linux_like() -> bool {
    // anything unix that isn't macos is considered "Linux" by AWS
    std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "unix"
        && std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "macos"
}
