fn main() {
    let config = Config::determine();
    eprintln!("build config: {config:?}");

    let mut builder = aws_c_builder::Builder::new("aws-c-io");
    if config.use_s2n {
        builder.dependency("S2N_TLS");
        builder.source_path("s2n");
        builder.cc_callback(|build| {
            build.define("USE_S2N", None);
        });
    }

    builder
        .source_paths(config.source_dirs.iter().copied())
        .cc_callback(|build| config.event_loop.set_as_define(build))
        .dependencies(["AWS_C_CAL", "AWS_C_COMMON"])
        .bindings_suffix(config.bindings.suffix())
        .build();
}

#[derive(Debug, Default)]
struct Config {
    use_s2n: bool,
    event_loop: EventLoop,
    source_dirs: &'static [&'static str],
    bindings: Bindings,
}

impl Config {
    fn determine() -> Self {
        let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
        if target_family == "windows" {
            return Self {
                use_s2n: false,
                event_loop: EventLoop::IoCompletionPorts,
                bindings: Bindings::Win32IoCompletionPorts,
                source_dirs: &["windows", "windows/iocp"],
            };
        }

        let target_vendor = std::env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
        if target_vendor == "apple" {
            return Self {
                use_s2n: false,
                event_loop: EventLoop::KQueue,
                bindings: Bindings::Apple,
                source_dirs: &["bsd", "posix", "darwin"],
            };
        }

        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        match target_os.as_str() {
            "linux" | "android" => Self {
                use_s2n: true,
                event_loop: EventLoop::EPoll,
                bindings: Bindings::Generic,
                source_dirs: &["linux", "posix"],
            },
            "freebsd" | "openbsd" | "netbsd" => Self {
                use_s2n: true,
                event_loop: EventLoop::KQueue,
                bindings: Bindings::Generic,
                source_dirs: &["bsd", "posix"],
            },
            _ => Self::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum EventLoop {
    #[default]
    None,
    IoCompletionPorts,
    EPoll,
    KQueue,
}

impl EventLoop {
    const fn define_value(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::IoCompletionPorts => Some("IO_COMPLETION_PORTS"),
            Self::EPoll => Some("EPOLL"),
            Self::KQueue => Some("KQUEUE"),
        }
    }

    fn set_as_define(self, build: &mut aws_c_builder::cc::Build) {
        if let Some(value) = self.define_value() {
            build.define(&format!("AWS_USE_{value}"), None);
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum Bindings {
    #[default]
    Generic,
    Win32IoCompletionPorts,
    Apple,
}

impl Bindings {
    const fn suffix(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Win32IoCompletionPorts => "win32_iocp",
            Self::Apple => "apple",
        }
    }
}
