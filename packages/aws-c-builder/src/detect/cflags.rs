#[derive(Debug)]
pub struct CommonProperties {
    has_stdint: bool,
    has_stdbool: bool,
    have_sysconf: bool,
    compiler_specific: CompilerSpecific,
}

impl CommonProperties {
    pub fn apply(&self, build: &mut cc::Build) {
        if !self.has_stdint {
            build.define("NO_STDINT", None);
        }
        if !self.has_stdbool {
            build.define("NO_STDBOOL", None);
        }
        if self.have_sysconf {
            build.define("HAVE_SYSCONF", None);
        }
        // TODO: set debug_build
        // TODO: set enable_tracing
        build.std("c99");
    }

    pub fn have_posix_large_file_support(&self) -> bool {
        match &self.compiler_specific {
            CompilerSpecific::Gnu { posix_lfs, .. } => posix_lfs.supported,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum CompilerSpecific {
    Msvc,
    Gnu {
        outline_atomics: bool,
        posix_lfs: PosixLfs,
    },
}

impl CompilerSpecific {
    fn apply(&self, build: &mut cc::Build) {
        match self {
            Self::Msvc => {
                build.flag("/volatile:iso").flag("/wd4204").flag("/wd4221");
            }
            Self::Gnu {
                outline_atomics,
                posix_lfs,
            } => {
                build.flag("-Wstrict-prototypes").flag_if_supported("-fPIC");
                if *outline_atomics {
                    build.flag("-moutline-atomics");
                }
                posix_lfs.apply(build);
            }
        }
    }
}

#[derive(Debug)]
struct PosixLfs {
    supported: bool,
    via_define: bool,
}

impl PosixLfs {
    fn apply(&self, build: &mut cc::Build) {
        if self.via_define {
            build.define("_FILE_OFFSET_BITS", "64");
        }
    }
}
