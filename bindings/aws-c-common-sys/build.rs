use std::path::{Path, PathBuf};

// TODO: static log level
// TODO: HAVE_SYSCONF

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let config = Config::determine(&out_dir);
    eprintln!("build config: {config:#?}");

    let src_include_dir = Path::new("aws-c-common/include");
    let generated_include_dir = out_dir.join("generated/include");
    prepare_headers(&config, src_include_dir, &generated_include_dir);

    aws_c_builder::Builder::new("aws-c-common")
        .include_dir(generated_include_dir.as_path())
        .source_path("external")
        .source_paths(config.arch.source_paths().iter().copied())
        .source_paths(config.platform.source_paths().iter().copied())
        .bindings_suffix(config.bindings.suffix())
        .cc_callback(|build| {
            config.thread_affinity_method.apply(build);
            config.thread_name_method.apply(build);
            config.platform.apply_defines(build);
            if config.use_simd_encoding {
                // TODO: HAVE_MM256_EXTRACT_EPI64
                build
                    .define("USE_SIMD_ENCODING", None)
                    .file("aws-c-common/source/arch/intel/encoding_avx2.c");
            }
            // not really relevant because we're not building a shared lib, but let's be
            // consistent.
            build.define("CJSON_HIDE_SYMBOLS", None);
        })
        .build();
}

fn prepare_headers(config: &Config, src_include_dir: &Path, out_include_dir: &Path) {
    fn copy_headers(src_include_dir: &Path, out_include_dir: &Path) {
        for entry in src_include_dir.read_dir().expect("read include dir") {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            let path = entry.path();
            let rel_path = path.strip_prefix(src_include_dir).unwrap();
            if file_type.is_dir() {
                let target_dir = out_include_dir.join(rel_path);
                std::fs::create_dir_all(&target_dir).unwrap();
                copy_headers(&path, &target_dir);
                continue;
            }
            let is_header = file_type.is_file()
                && path.extension().is_some_and(|ext| {
                    ext.eq_ignore_ascii_case("h") || ext.eq_ignore_ascii_case("inl")
                });
            if !is_header {
                continue;
            }

            std::fs::copy(&path, out_include_dir.join(rel_path)).unwrap();
        }
    }

    copy_headers(src_include_dir, out_include_dir);
    let config_template = src_include_dir.join("aws/common/config.h.in");
    let generated_config_header = out_include_dir.join("aws/common/config.h");
    config
        .config_header
        .prepare(&config_template, &generated_config_header);
}

#[derive(Debug)]
struct Config {
    bindings: Bindings,
    config_header: ConfigHeader,
    arch: Arch,
    platform: Platform,
    use_simd_encoding: bool,
    thread_affinity_method: aws_c_builder::detect::thread::AffinityMethod,
    thread_name_method: aws_c_builder::detect::thread::NameMethod,
}

impl Config {
    fn determine(out_dir: &Path) -> Self {
        let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
        let target_vendor = std::env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let target_features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap();
        let target_features = target_features.split(',').collect::<Box<_>>();
        let compiler = aws_c_builder::cc::Build::new().get_compiler();

        Self {
            bindings: Bindings::determine(&target_os),
            config_header: ConfigHeader::determine(out_dir, &target_family),
            arch: Arch::determine(&target_arch, compiler.is_like_msvc()),
            platform: Platform::determine(&target_os),
            use_simd_encoding: target_features.contains(&"avx2"),
            thread_affinity_method: aws_c_builder::detect::thread::AffinityMethod::detect(
                out_dir,
                &target_family,
                &target_os,
            ),
            thread_name_method: aws_c_builder::detect::thread::NameMethod::detect(
                out_dir,
                &target_family,
                &target_vendor,
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum Platform {
    #[default]
    Posix,
    Android,
    Linux,
    Windows,
}

impl Platform {
    fn determine(target_os: &str) -> Self {
        match target_os {
            "android" => Self::Android,
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            _ => Self::Posix,
        }
    }

    const fn source_paths(self) -> &'static [&'static str] {
        const SYSTEM_INFO_FALLBACK: &str = "platform_fallback_stubs/system_info.c";
        match self {
            Self::Posix => &["posix", SYSTEM_INFO_FALLBACK],
            Self::Android => &["posix", "android", SYSTEM_INFO_FALLBACK],
            Self::Linux => &["posix", "linux"],
            Self::Windows => &["windows", SYSTEM_INFO_FALLBACK],
        }
    }

    fn apply_defines(self, build: &mut aws_c_builder::cc::Build) {
        match self {
            Self::Windows => {
                build
                    .define("WINDOWS_KERNEL_LIB", "Kernel32")
                    .define("PSAPI_VERSION", "1");
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum Arch {
    #[default]
    Generic,
    ArmAsm,
    ArmMsvc,
    IntelAsm,
    IntelMsvc,
}

impl Arch {
    fn determine(target_arch: &str, is_msvc: bool) -> Self {
        match target_arch {
            "x86" | "x86_64" => {
                if is_msvc {
                    Self::IntelMsvc
                } else {
                    Self::IntelAsm
                }
            }
            "arm" | "aarch64" => {
                if is_msvc {
                    Self::ArmMsvc
                } else {
                    Self::ArmAsm
                }
            }
            _ => Self::Generic,
        }
    }

    const fn source_paths(self) -> &'static [&'static str] {
        match self {
            Self::Generic => &["arch/generic"],
            Self::ArmAsm => &["arch/arm/asm"],
            Self::ArmMsvc => &["arch/arm/msvc"],
            Self::IntelAsm => &["arch/intel/asm", "arch/intel/cpuid.c"],
            Self::IntelMsvc => &["arch/intel/msvc", "arch/intel/cpuid.c"],
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum Bindings {
    #[default]
    Generic,
    Apple,
    Windows,
}

impl Bindings {
    fn determine(target_os: &str) -> Self {
        match target_os {
            "macos" | "ios" => Self::Apple,
            "windows" => Self::Windows,
            _ => Self::Generic,
        }
    }

    const fn suffix(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Apple => "apple",
            Self::Windows => "windows",
        }
    }
}

#[derive(Debug)]
struct ConfigHeader {
    gcc_overflow_math_extensions: bool,
    gcc_inline_asm: bool,
    msvc_intrinsics_x64: bool,
    posix_large_file_support: bool,
    execinfo: bool,
    winapi_desktop: bool,
    linux_if_link_h: bool,
}

impl ConfigHeader {
    fn determine(out_dir: &Path, target_family: &str) -> Self {
        Self {
            gcc_overflow_math_extensions: false, /* requires runtime test for some reason which obviously doesn't work when cross-compiling */
            gcc_inline_asm: aws_c_builder::detect::have_gcc_inline_asm(out_dir),
            msvc_intrinsics_x64: false, // todo
            posix_large_file_support: aws_c_builder::detect::have_posix_lfs(out_dir),
            execinfo: aws_c_builder::detect::have_execinfo(out_dir),
            winapi_desktop: aws_c_builder::detect::have_winapi_desktop(out_dir, target_family),
            linux_if_link_h: aws_c_builder::detect::have_linux_if_link_h(out_dir),
        }
    }

    fn resolve_define(&self, name: &str) -> Option<bool> {
        match name {
            "AWS_HAVE_GCC_OVERFLOW_MATH_EXTENSIONS" => Some(self.gcc_overflow_math_extensions),
            "AWS_HAVE_GCC_INLINE_ASM" => Some(self.gcc_inline_asm),
            "AWS_HAVE_MSVC_INTRINSICS_X64" => Some(self.msvc_intrinsics_x64),
            "AWS_HAVE_POSIX_LARGE_FILE_SUPPORT" => Some(self.posix_large_file_support),
            "AWS_HAVE_EXECINFO" => Some(self.execinfo),
            "AWS_HAVE_WINAPI_DESKTOP" => Some(self.winapi_desktop),
            "AWS_HAVE_LINUX_IF_LINK_H" => Some(self.linux_if_link_h),
            _ => None,
        }
    }

    /// Renders a single #cmakedefine line.
    ///
    /// Defines with values (ex. `#cmakedefine FOO_STRING "@FOO_STRING@"`) are
    /// not supported.
    ///
    /// Syntax: <https://cmake.org/cmake/help/latest/command/configure_file.html>
    fn render_template_define(
        &self,
        define_args: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut define_args = define_args.split_whitespace();
        let define_name = define_args.next().ok_or("missing VAR name")?;
        if define_args.next().is_some() {
            return Err("extended #cmakedefine not supported".into());
        }

        let value = self
            .resolve_define(define_name)
            .ok_or_else(|| format!("unknown define name: {define_name:?}"))?;
        if value {
            Ok(format!("#define {define_name}"))
        } else {
            Ok(format!("/* #undef {define_name} */"))
        }
    }

    /// Renders a cmake configuration header file by replacing all the config
    /// template lines.
    ///
    /// See: <https://cmake.org/cmake/help/latest/command/configure_file.html>
    fn render_template(&self, template: &str) -> String {
        const DEFINE_MARKER: &str = "#cmakedefine ";

        let mut output = String::with_capacity(template.len());
        for line in template.lines() {
            if let Some(define_args) = line.strip_prefix(DEFINE_MARKER) {
                match self.render_template_define(define_args) {
                    Ok(line) => output.push_str(&line),
                    Err(err) => panic!("{err}\nline: {line:?}"),
                }
            } else {
                output.push_str(line);
            }
            output.push('\n');
        }
        output
    }

    fn prepare(&self, src: &Path, out: &Path) {
        let template = std::fs::read_to_string(src).expect("read config header template");
        let output = self.render_template(&template);
        std::fs::write(out, output).expect("write config header");
    }
}
