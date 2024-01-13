use std::path::Path;

mod cflags;
mod feature_tests;
mod simd;
mod thread_affinity;
mod thread_name;

/// Checks whether the given code snippet successfully compiles.
///
/// Must not be called in parallel.
pub fn check_compiles(out_dir: &Path, code: &str) -> bool {
    check_compiles_with_cc(out_dir, &mut cc::Build::new(), code)
}

/// Checks whether the given code snippet successfully compiles with a
/// pre-configured [`cc::Build`].
///
/// Must not be called in parallel.
pub fn check_compiles_with_cc(out_dir: &Path, build: &mut cc::Build, code: &str) -> bool {
    let c_file = out_dir.join("compile_test.c");
    std::fs::write(&c_file, code).expect("write c code compilation test code");
    // TODO: this is too noisy
    build
        .cargo_metadata(false)
        .emit_rerun_if_env_changed(false)
        .warnings(false)
        .extra_warnings(false)
        .opt_level(0)
        .file(c_file)
        .try_compile_intermediates()
        .is_ok()
}

/// Checks whether a given symbol is available during compilation of C code.
///
/// Based on cmake's implementation.
/// See: <https://github.com/Kitware/CMake/blob/master/Modules/CheckSymbolExists.cmake>
pub fn check_symbol_exists<H>(out_dir: &Path, headers: H, symbol: &str) -> bool
where
    H: IntoIterator,
    H::Item: AsRef<str>,
{
    use std::fmt::Write;
    let mut code = String::new();
    for header in headers {
        writeln!(code, "#include <{}>", header.as_ref()).unwrap();
    }
    write!(
        code,
        "
int main(int argc, char** argv) {{
    (void)argv;
    #ifndef {symbol}
    return ((int*)(&{symbol}))[argc];
    #else
    (void)argc;
    return 0;
    #endif
}}
"
    )
    .unwrap();
    check_compiles(out_dir, &code)
}

/// Checks whether a given header is available during compilation.
///
/// See: <https://github.com/Kitware/CMake/blob/master/Modules/CheckIncludeFile.cmake>
pub fn check_include_file(out_dir: &Path, name: &str) -> bool {
    let code = format!(
        r#"
#include <{name}>
int main(void) {{ return 0; }}
"#
    );
    check_compiles(out_dir, &code)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Debug,
    Release,
}

impl Profile {
    pub fn from_env() -> Self {
        match std::env::var("PROFILE").as_deref() {
            Ok("debug") => Self::Debug,
            _ => Self::Release,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetFamily {
    Other,
    Unix,
    Windows,
}

impl TargetFamily {
    pub fn from_env() -> Self {
        match std::env::var("CARGO_CFG_TARGET_FAMILY").as_deref() {
            Ok("unix") => Self::Unix,
            Ok("windows") => Self::Windows,
            _ => Self::Other,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetOs {
    Other,
    Windows,
    Macos,
    Ios,
    Linux,
    Android,
    Freebsd,
    Dragonfly,
    Openbsd,
    Netbsd,
}

impl TargetOs {
    pub fn from_env() -> Self {
        match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
            Ok("windows") => Self::Windows,
            Ok("macos") => Self::Macos,
            Ok("ios") => Self::Ios,
            Ok("linux") => Self::Linux,
            Ok("android") => Self::Android,
            Ok("freebsd") => Self::Freebsd,
            Ok("dragonfly") => Self::Dragonfly,
            Ok("openbsd") => Self::Openbsd,
            Ok("netbsd") => Self::Netbsd,
            _ => Self::Other,
        }
    }

    pub const fn is_bsd(self) -> bool {
        matches!(self, Self::Freebsd | Self::Openbsd | Self::Netbsd)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetVendor {
    Other,
    Apple,
}

impl TargetVendor {
    pub fn from_env() -> Self {
        match std::env::var("CARGO_CFG_TARGET_VENDOR").as_deref() {
            Ok("apple") => Self::Apple,
            _ => Self::Other,
        }
    }
}
