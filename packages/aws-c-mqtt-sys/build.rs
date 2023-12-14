const LIB_NAME: &str = "aws-c-mqtt";

fn main() {
    let cmake_roots = [
        std::env::var("DEP_AWS_C_CAL_ROOT").unwrap(),
        std::env::var("DEP_AWS_C_COMMON_ROOT").unwrap(),
        std::env::var("DEP_AWS_C_COMPRESSION_ROOT").unwrap(),
        std::env::var("DEP_AWS_C_HTTP_ROOT").unwrap(),
        std::env::var("DEP_AWS_C_IO_ROOT").unwrap(),
    ];

    println!("cargo:rerun-if-changed={LIB_NAME}");
    let out_dir = cmake::Config::new(LIB_NAME)
        .define("CMAKE_PREFIX_PATH", cmake_roots.join(";"))
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_TESTING", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();
    println!("cargo:rustc-link-search=native={out_dir}/lib");
    println!("cargo:rustc-link-lib=static={LIB_NAME}");
}
