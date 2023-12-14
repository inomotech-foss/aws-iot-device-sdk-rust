const LIB_NAME: &str = "aws-c-common";

fn main() {
    println!("cargo:rerun-if-changed={LIB_NAME}");
    let out_dir = cmake::Config::new(LIB_NAME)
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_TESTING", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    println!("cargo:rustc-link-lib=static={LIB_NAME}");
}
