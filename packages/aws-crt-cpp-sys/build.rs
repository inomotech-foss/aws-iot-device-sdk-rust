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
    "crypto",
    "s2n",
];

fn main() {
    println!("cargo:rerun-if-changed=aws-crt-cpp");
    let out_dir = cmake::Config::new("aws-crt-cpp")
        .define("AWS_ENABLE_LTO", "ON")
        .define("BUILD_DEPS", "ON") // we don't want to build all of the c libs ourselves
        .define("BUILD_TESTING", "OFF")
        .define("USE_OPENSSL", "OFF")
        .build();
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    for lib in LINK_LIBS {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}
