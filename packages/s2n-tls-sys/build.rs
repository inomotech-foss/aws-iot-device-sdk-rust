fn main() {
    aws_c_builder::Config::new("s2n-tls")
        .aws_dependencies(&["AWS_LC"])
        .cmake_callback(|config| {
            config
                .define("SEARCH_LIBCRYPTO", "OFF")
                .define("S2N_LTO", "ON");
        })
        .link_libraries(&["s2n"])
        .bindgen_callback(|builder| builder.allowlist_file(".+/s2n.h"))
        .build()
}
