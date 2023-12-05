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
        println!("cargo:rerun-if-changed=aws-crt-cpp");
        let mut config = cmake::Config::new("aws-crt-cpp");
        config
            .define("BUILD_DEPS", "ON")
            .define("USE_OPENSSL", "ON");
        if let Some(openssl_root) = std::env::var_os("DEP_OPENSSL_ROOT") {
            config.define("OpenSSL_ROOT", openssl_root);
        }
        config.build()
    };
    let out_dir = out_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    for lib in LINK_LIBS {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    println!("cargo:lib={out_dir}/lib");
    println!("cargo:include={out_dir}/include");
}
