fn main() {
    let crt_root = std::env::var("DEP_AWS_CRT_CPP_ROOT").unwrap();

    println!("cargo:rerun-if-changed=aws-c-iot");
    let out_dir = cmake::Config::new("aws-c-iot")
        .define("CMAKE_PREFIX_PATH", crt_root)
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_TESTING", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    println!("cargo:rustc-link-lib=static=aws-c-iot");
}
