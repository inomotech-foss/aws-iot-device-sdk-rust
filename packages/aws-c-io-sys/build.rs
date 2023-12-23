fn main() {
    let mut aws_deps = vec!["AWS_C_CAL", "AWS_C_COMMON"];
    if aws_c_builder::is_linux_like() {
        aws_deps.push("S2N");
    }
    aws_c_builder::Config::new("aws-c-io")
        .aws_dependencies(aws_deps)
        .bindgen_callback(|builder| {
            builder
                .allowlist_item("(?i)aws_async_input_stream.*")
                .allowlist_item("(?i)aws_channel.*")
                .allowlist_item("(?i)aws_client_bootstrap.*")
                .allowlist_item("(?i)aws_custom_key_op_handler.*")
                .allowlist_item("(?i)aws_event_loop.*")
                .allowlist_item("(?i)aws_exponential_backoff.*")
                .allowlist_item("(?i)aws_future.*")
                .allowlist_item("(?i)aws_host.*")
                .allowlist_item("(?i)aws_input_stream.*")
                .allowlist_item("(?i)aws_(c_)?io.*")
                .allowlist_item("(?i)aws_pkcs11_lib.*")
                .allowlist_item("(?i)aws_retry.*")
                .allowlist_item("(?i)aws_server.*")
                .allowlist_item("(?i)aws_socket.*")
                .allowlist_item("(?i)aws_stream.*")
                .allowlist_item("(?i)aws_tls.*")
                .allowlist_type("aws_address_record_type")
                .allowlist_type("aws_generate_random_fn")
                .allowlist_type("aws_new_event_loop_fn")
                .allowlist_type("aws_standard_retry_options")
                .allowlist_type("aws.+host.+_fn")
        })
        .build();
}
