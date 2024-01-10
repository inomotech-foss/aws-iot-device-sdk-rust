#[derive(Debug)]
pub struct FeatureTests {
    pub have_gcc_overflow_math_extensions: bool,
    pub have_winapi_desktop: bool,
    pub have_gcc_inline_asm: bool,
    pub have_auxv: bool,
    pub have_execinfo: bool,
    pub have_linux_if_link_h: bool,
    pub have_msvc_intrinsics_x64: bool,
}

#[derive(Debug)]
enum CFlags {
    None,
    Msvc,
    Gnu,
}
