use std::path::Path;

// TODO NO_STDBOOL and NO_STDINT

pub mod thread;

pub fn have_winapi_desktop(out_dir: &Path, target_family: &str) -> bool {
    if target_family != "windows" {
        return false;
    }
    check_compiles(
        out_dir,
        r#"
#include <Windows.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
int main() { return 0; }
#else
it's not windows desktop
#endif
"#,
    )
}

pub fn have_gcc_inline_asm(out_dir: &Path) -> bool {
    check_compiles(
        out_dir,
        r#"
int main() {
    int foo = 42, bar = 24;
    __asm__ __volatile__("":"=r"(foo):"r"(bar):"memory");
}
"#,
    )
}

pub fn have_execinfo(out_dir: &Path) -> bool {
    check_compiles(
        out_dir,
        r#"
#include <execinfo.h>
#include <stdlib.h>
int main() {
    backtrace(NULL, 0);
    return 0;
}
"#,
    )
}

pub fn have_linux_if_link_h(out_dir: &Path) -> bool {
    check_compiles(
        out_dir,
        r#"
#include <linux/if_link.h>
int main() { return 1; }
"#,
    )
}

// TODO: there's a way to force posix lfs using a define, but that requires
// setting a define as part of the compilation so we need to do that higher up.
// See the AWS impl for reference.
pub fn have_posix_lfs(out_dir: &Path) -> bool {
    check_compiles(
        out_dir,
        r#"
#include <stdio.h>

/* fails to compile if off_t smaller than 64bits */
typedef char array[sizeof(off_t) >= 8 ? 1 : -1];

int main() { return 0; }
"#,
    )
}

pub fn have_sysconf(out_dir: &Path) -> bool {
    check_compiles(
        out_dir,
        r#"
#include <unistd.h>
int main() { sysconf(_SC_NPROCESSORS_ONLN); }
"#,
    )
}

/// Checks whether the given code snippet successfully compiles.
///
/// Must not be called in parallel.
pub fn check_compiles(out_dir: &Path, code: &str) -> bool {
    let c_file = out_dir.join("compile_test.c");
    std::fs::write(&c_file, code).expect("write c code compilation test code");

    // TODO: this is too noisy
    cc::Build::new()
        .cargo_metadata(false)
        .emit_rerun_if_env_changed(false)
        .warnings(false)
        .extra_warnings(false)
        .opt_level(0)
        .file(c_file)
        .try_compile("compile_test")
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
