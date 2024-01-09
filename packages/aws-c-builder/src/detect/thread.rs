use std::path::Path;

use super::{check_compiles, check_symbol_exists};

#[derive(Clone, Copy, Debug, Default)]
pub enum AffinityMethod {
    #[default]
    None,
    PthreadAttr,
    Pthread,
}

impl AffinityMethod {
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

#[derive(Debug)]
pub struct NameMethod {
    setter: Option<NameSetter>,
    getter: Option<NameGetter>,
}

impl NameMethod {
    pub fn detect(out_dir: &Path, target_family: &str, target_vendor: &str) -> Self {
        if target_family == "windows" {
            // On Windows we do a runtime check for both getter and setter, instead of
            // compile-time check
            return Self {
                setter: None,
                getter: None,
            };
        }

        Self {
            setter: NameSetter::detect(out_dir, target_vendor),
            getter: NameGetter::detect(out_dir, target_vendor),
        }
    }

    pub fn apply(&self, build: &mut cc::Build) {
        if let Some(setter) = self.setter {
            build.define(setter.define_name(), None);
        }
        if let Some(getter) = self.getter {
            build.define(getter.define_name(), None);
        }
    }

    fn check_compiles(out_dir: &Path, call: &str) -> bool {
        let code = format!(
            r#"
#define _GNU_SOURCE
#include <pthread.h>

#if defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__)
#include <pthread_np.h>
#endif

int main() {{
    pthread_t thread_id;
    {call}
}}
"#
        );
        check_compiles(out_dir, &code)
    }
}

#[derive(Clone, Copy, Debug)]
enum NameSetter {
    Setname2,
    Setname3,
    SetName2,
}

impl NameSetter {
    fn detect(out_dir: &Path, target_vendor: &str) -> Option<Self> {
        if target_vendor == "apple" {
            // All Apple platforms we support have 1 arg version of the function.
            // So skip compile time check here and instead check if its apple in
            // the thread code.
            return None;
        }

        // pthread_setname_np() usually takes 2 args
        if NameMethod::check_compiles(out_dir, r#"pthread_setname_np(thread_id, "asdf");"#) {
            return Some(Self::Setname2);
        }
        // OpenBSD's function takes 2 args, but has a different name.
        if NameMethod::check_compiles(out_dir, r#"pthread_set_name_np(thread_id, "asdf");"#) {
            return Some(Self::SetName2);
        }
        // But on NetBSD it takes 3!
        if NameMethod::check_compiles(out_dir, r#"pthread_setname_np(thread_id, "asdf", NULL);"#) {
            return Some(Self::Setname3);
        }

        // And on many older/weirder platforms it's just not supported
        // Consider using prctl if we really want to support those
        None
    }

    const fn define_name(self) -> &'static str {
        match self {
            Self::Setname2 => "AWS_PTHREAD_SETNAME_TAKES_2ARGS",
            Self::Setname3 => "AWS_PTHREAD_SETNAME_TAKES_3ARGS",
            Self::SetName2 => "AWS_PTHREAD_SET_NAME_TAKES_2ARGS",
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum NameGetter {
    Getname3,
    Getname2,
    GetName2,
}

impl NameGetter {
    fn detect(out_dir: &Path, target_vendor: &str) -> Option<Self> {
        if target_vendor == "apple" {
            // All Apple platforms we support have the same function, so no need for
            // compile-time check.
            return Some(Self::Getname3);
        }

        // Some platforms have 2 arg version
        if NameMethod::check_compiles(
            out_dir,
            r#"char name[16] = {0}; pthread_getname_np(thread_id, name);"#,
        ) {
            return Some(Self::Getname2);
        }
        // Some platforms have 2 arg version but with a different name (eg, OpenBSD)
        if NameMethod::check_compiles(
            out_dir,
            r#"char name[16] = {0}; pthread_get_name_np(thread_id, name);"#,
        ) {
            return Some(Self::GetName2);
        }
        // But majority have 3
        if NameMethod::check_compiles(
            out_dir,
            r#"char name[16] = {0}; pthread_getname_np(thread_id, name, 16);"#,
        ) {
            return Some(Self::Getname3);
        }

        None
    }

    const fn define_name(self) -> &'static str {
        match self {
            Self::Getname3 => "AWS_PTHREAD_GETNAME_TAKES_3ARGS",
            Self::Getname2 => "AWS_PTHREAD_GETNAME_TAKES_2ARGS",
            Self::GetName2 => "AWS_PTHREAD_GET_NAME_TAKES_2ARGS",
        }
    }
}
