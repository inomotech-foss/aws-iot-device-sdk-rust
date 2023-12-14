const LIB_NAME: &str = "aws-c-cal";

fn main() {
    let common_root = std::env::var("DEP_AWS_C_COMMON_ROOT").unwrap();

    println!("cargo:rerun-if-changed={LIB_NAME}");
    let out_dir = cmake::Config::new(LIB_NAME)
        .define("CMAKE_PREFIX_PATH", common_root)
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_TESTING", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();
    println!("cargo:rustc-link-search=native={out_dir}/lib");
    println!("cargo:rustc-link-lib=static={LIB_NAME}");
}
