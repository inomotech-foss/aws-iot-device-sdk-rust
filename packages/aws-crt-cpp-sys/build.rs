const LINK_LIBS: &[&str] = &[
    "aws-c-auth",
    "aws-c-cal",
    "aws-c-common",
    "aws-c-compression",
    "aws-c-event-stream",
    "aws-c-http",
    "aws-c-io",
    "aws-c-mqtt",
    "aws-c-s3",
    "aws-c-sdkutils",
    "aws-checksums",
    "aws-crt-cpp",
];

const LINUX_EXTRA_LINK_LIBS: &[&str] = &["crypto", "s2n"];

fn determine_link_libs() -> impl Iterator<Item = &'static str> {
    let is_linux = std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "linux";

    let extra_libs = if is_linux { LINUX_EXTRA_LINK_LIBS } else { &[] };
    LINK_LIBS
        .iter()
        .copied()
        .chain(extra_libs.into_iter().copied())
}

fn main() {
    println!("cargo:rerun-if-changed=aws-crt-cpp");
    let out_dir = cmake::Config::new("aws-crt-cpp")
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_DEPS", "OFF") // we don't want to build all of the c libs ourselves
        .define("BUILD_TESTING", "OFF")
        .define("USE_OPENSSL", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    for lib in determine_link_libs() {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}
