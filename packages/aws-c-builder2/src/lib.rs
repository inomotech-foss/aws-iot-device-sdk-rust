use std::path::{Path, PathBuf};

pub use cc;

mod bindings;
mod compile;

pub struct Builder<'a> {
    lib_dir: PathBuf,
    dependencies: Vec<&'a str>,
    source_subdirs: Vec<&'a str>,
    cc_callbacks: Vec<Box<dyn FnMut(&mut cc::Build) + 'a>>,
    bindings_suffix: &'a str,
}

impl<'a> Builder<'a> {
    pub fn new(lib_dir: impl AsRef<Path>) -> Self {
        let lib_dir = lib_dir.as_ref().canonicalize().expect("lib dir");
        Self {
            lib_dir,
            dependencies: Vec::new(),
            source_subdirs: Vec::new(),
            cc_callbacks: Vec::new(),
            bindings_suffix: "",
        }
    }

    pub fn dependencies(&mut self, iter: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.dependencies.extend(iter);
        self
    }

    pub fn dependency(&mut self, value: &'a str) -> &mut Self {
        self.dependencies.push(value);
        self
    }

    pub fn source_subdirs(&mut self, iter: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.source_subdirs.extend(iter);
        self
    }

    pub fn source_subdir(&mut self, value: &'a str) -> &mut Self {
        self.source_subdirs.push(value);
        self
    }

    pub fn cc_callback(&mut self, cb: impl FnMut(&mut cc::Build) + 'a) -> &mut Self {
        self.cc_callbacks.push(Box::new(cb));
        self
    }

    pub fn bindings_suffix(&mut self, value: &'a str) -> &mut Self {
        self.bindings_suffix = value;
        self
    }

    pub fn build(&mut self) {
        let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

        let include_dir = self.lib_dir.join("include");
        println!("cargo:include={}", include_dir.to_str().unwrap());

        let include_dirs =
            std::iter::once(include_dir)
                .chain(self.dependencies.iter().map(|name| {
                    PathBuf::from(std::env::var(format!("DEP_{name}_INCLUDE")).unwrap())
                }))
                .collect::<Vec<_>>();

        self::compile::run(self, &include_dirs);
        self::bindings::prepare(&out_dir, &include_dirs, self.bindings_suffix);
    }
}
