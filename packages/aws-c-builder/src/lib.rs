use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub use cc;

mod bindings;
mod compile;
pub mod detect;

const ENABLE_TRACING_FEATURE_ENV: &str = "CARGO_FEATURE_ENABLE_TRACING";

type BoxedCcCallback<'a> = Box<dyn FnMut(&mut cc::Build) + 'a>;

pub struct Builder<'a> {
    lib_dir: PathBuf,
    dependencies: Vec<&'a str>,
    source_paths: Vec<Cow<'a, Path>>,
    include_dir: Option<Cow<'a, Path>>,
    cc_callbacks: Vec<BoxedCcCallback<'a>>,
    bindings_suffix: &'a str,
}

impl<'a> Builder<'a> {
    pub fn new(lib_dir: impl AsRef<Path>) -> Self {
        let lib_dir = lib_dir.as_ref().canonicalize().expect("lib dir");
        Self {
            lib_dir,
            dependencies: Vec::new(),
            source_paths: Vec::new(),
            include_dir: None,
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

    pub fn source_paths<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsCow<'a, Path>,
    {
        self.source_paths
            .extend(iter.into_iter().map(AsCow::as_cow_path));
        self
    }

    pub fn source_path(&mut self, value: impl AsCow<'a, Path>) -> &mut Self {
        self.source_paths.push(value.as_cow_path());
        self
    }

    pub fn include_dir(&mut self, value: impl AsCow<'a, Path>) -> &mut Self {
        assert!(self.include_dir.is_none());
        self.include_dir = Some(value.as_cow_path());
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

        let include_dir = self
            .include_dir
            .clone()
            .unwrap_or_else(|| Cow::Owned(self.lib_dir.join("include")));
        println!("cargo:include={}", include_dir.to_str().unwrap());

        let include_dirs = std::iter::once(include_dir)
            .chain(
                self.dependencies
                    .iter()
                    .map(|name| get_dep_include_path(name))
                    .map(Cow::from),
            )
            .collect::<Vec<_>>();
        let enable_tracing = should_enable_tracing();

        self::compile::run(self, &include_dirs, enable_tracing);
        self::bindings::prepare(&out_dir, &include_dirs, self.bindings_suffix);
    }
}

pub trait AsCow<'a, T>
where
    T: ToOwned + ?Sized,
{
    fn as_cow_path(self) -> Cow<'a, T>;
}

impl<'a, T> AsCow<'a, T> for Cow<'a, T>
where
    T: ToOwned + ?Sized,
{
    fn as_cow_path(self) -> Cow<'a, T> {
        self
    }
}

impl<'a, T> AsCow<'a, T> for &'a T
where
    T: ToOwned + ?Sized,
{
    fn as_cow_path(self) -> Cow<'a, T> {
        Cow::Borrowed(self)
    }
}

impl<'a, T> AsCow<'a, T> for T
where
    T: Clone,
{
    fn as_cow_path(self) -> Cow<'a, T> {
        Cow::Owned(self)
    }
}

impl<'a> AsCow<'a, Path> for &'a str {
    fn as_cow_path(self) -> Cow<'a, Path> {
        Cow::Borrowed(Path::new(self))
    }
}

fn get_dep_include_path(name: &str) -> PathBuf {
    let Some(raw) = std::env::var_os(format!("DEP_{name}_INCLUDE")) else {
        panic!("dependency {name} didn't set 'include' variable");
    };
    PathBuf::from(raw)
}

fn should_enable_tracing() -> bool {
    std::env::var_os(ENABLE_TRACING_FEATURE_ENV).is_some()
}
