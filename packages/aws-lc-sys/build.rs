fn main() {
    aws_c_builder::Config::new("aws-lc")
        .cmake_callback(|config| {
            config
                .define("BUILD_TOOL", "OFF")
                .define("DISABLE_PERL", "ON")
                .define("DISABLE_GO", "ON");
        })
        .link_libraries(&["crypto", "ssl"])
        .bindgen_blanket_include_dirs(&["openssl"])
        .bindgen_callback(|builder| {
            builder
                .blocklist_item("BIO_vsnprintf")
                .blocklist_item("OPENSSL_vasprintf")
        })
        .build()
}
