fn main() {
    aws_c_builder::Config::new("aws-c-common")
        .bindgen_callback(|builder| {
            builder
                .opaque_type("aws_hash_table")
                .opaque_type("aws_log_formatter_vtable")
                .opaque_type("aws_thread_once")
                .allowlist_file(".+/aws/common/.+")
                .blocklist_item("aws_format_standard_log_line")
                .blocklist_item("aws_log_formatter_format_fn")
        })
        .build()
}
