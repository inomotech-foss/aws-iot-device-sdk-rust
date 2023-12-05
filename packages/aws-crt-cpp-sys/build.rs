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

fn main() {
    let out_dir = {
        let openssl_root = std::env::var_os("DEP_OPENSSL_ROOT");
        // we want to use openssl if the feature is explicitly enabled OR if openssl-sys
        // is present in the build tree.
        let use_openssl =
            std::env::var_os("CARGO_FEATURE_OPENSSL").is_some() || openssl_root.is_some();

        println!("cargo:rerun-if-changed=aws-crt-cpp");
        let mut config = cmake::Config::new("aws-crt-cpp");
        config
            .define("AWS_ENABLE_LTO", "ON")
            .define("BUILD_DEPS", "ON") // we don't want to build all of the c libs ourselves
            .define("BUILD_TESTING", "OFF")
            .define("USE_OPENSSL", cmake_bool(use_openssl));
        if let Some(openssl_root) = openssl_root {
            config.define("OpenSSL_ROOT", openssl_root);
        }
        config.build()
    };
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    for lib in LINK_LIBS {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}

fn cmake_bool(value: bool) -> &'static str {
    match value {
        true => "ON",
        false => "OFF",
    }
}
