use std::path::Path;

use super::check_symbol_exists;

#[derive(Clone, Copy, Debug, Default)]
pub enum ThreadAffinityMethod {
    #[default]
    None,
    PthreadAttr,
    Pthread,
}

impl ThreadAffinityMethod {
    pub fn detect(out_dir: &Path, target_family: &str, target_os: &str) -> Self {
        // Non-POSIX, Android, and Apple platforms do not support thread affinity.
        if target_family != "unix" {
            return Self::None;
        }

        // BSDs put nonportable pthread declarations in a separate header.
        let headers = if target_os.ends_with("bsd") {
            ["pthread.h", "pthread_np.h"].as_slice()
        } else {
            ["pthread.h"].as_slice()
        };

        // Using pthread attrs is the preferred method, but is glibc-specific.
        if check_symbol_exists(out_dir, headers, "pthread_attr_setaffinity_np") {
            return Self::PthreadAttr;
        }

        // This method is still nonportable, but is supported by musl and BSDs.
        if check_symbol_exists(out_dir, headers, "pthread_setaffinity_np") {
            return Self::Pthread;
        }

        // If we got here, we expected thread affinity support but didn't find it.
        // We still build with degraded NUMA performance, but show a warning.
        println!("cargo:warning=No supported method for setting thread affinity");
        Self::None
    }

    pub fn apply(self, build: &mut cc::Build) {
        build.define("AWS_AFFINITY_METHOD", self.define_value());
    }

    pub const fn define_value(self) -> &'static str {
        match self {
            Self::None => "AWS_AFFINITY_METHOD_NONE",
            Self::PthreadAttr => "AWS_AFFINITY_METHOD_PTHREAD_ATTR",
            Self::Pthread => "AWS_AFFINITY_METHOD_PTHREAD",
        }
    }
}
