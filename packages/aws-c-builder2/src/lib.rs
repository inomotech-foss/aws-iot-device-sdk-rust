use std::path::{Path, PathBuf};

mod bindings;
mod compile;

pub struct Builder<'a> {
    lib_dir: &'a Path,
    dependencies: Vec<&'a str>,
}

impl<'a> Builder<'a> {
    pub fn new(lib_dir: &'a Path) -> Self {
        Self {
            lib_dir,
            dependencies: Vec::new(),
        }
    }

    pub fn dependencies(&mut self, iter: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.dependencies.extend(iter);
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

        self::compile::run(&self.lib_dir, &include_dirs);
        self::bindings::prepare(&out_dir, &include_dirs);
    }
}
