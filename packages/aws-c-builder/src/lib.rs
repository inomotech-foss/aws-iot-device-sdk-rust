use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub use cc;

mod bindings;
mod compile;

pub struct Builder<'a> {
    lib_dir: PathBuf,
    dependencies: Vec<&'a str>,
    source_subdirs: Vec<&'a str>,
    extra_include_dirs: Vec<Cow<'a, Path>>,
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
            extra_include_dirs: Vec::new(),
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

    pub fn include_dirs(&mut self, iter: impl IntoIterator<Item = &'a Path>) -> &mut Self {
        self.extra_include_dirs
            .extend(iter.into_iter().map(Cow::from));
        self
    }

    pub fn include_dir(&mut self, value: &'a Path) -> &mut Self {
        self.extra_include_dirs.push(Cow::from(value));
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

        let include_dirs = std::iter::once(include_dir)
            .chain(
                self.dependencies
                    .iter()
                    .map(|name| get_dep_include_path(name)),
            )
            .map(Cow::from)
            .chain(self.extra_include_dirs.iter().cloned())
            .collect::<Vec<_>>();

        self::compile::run(self, &include_dirs);
        self::bindings::prepare(&out_dir, &include_dirs, self.bindings_suffix);
    }
}

fn get_dep_include_path(name: &str) -> PathBuf {
    let Some(raw) = std::env::var_os(format!("DEP_{name}_INCLUDE")) else {
        panic!("dependency {name} didn't set 'include' variable");
    };
    PathBuf::from(raw)
}
