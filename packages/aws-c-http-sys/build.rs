fn main() {
    aws_c_builder::Config::new("aws-c-http")
        .aws_dependencies(&["AWS_C_COMPRESSION", "AWS_C_IO"])
        .include_dir_names(&["http"])
        .bindgen_callback(|config| {
            config
                .blocklist_item("aws_hash_table")
                .blocklist_item("aws_tls_connection_options")
        })
        .build()
}
