fn main() {
    aws_c_builder::Config::new("aws-c-http")
        .aws_dependencies(&["AWS_C_COMPRESSION", "AWS_C_IO"])
        .bindgen_callback(|builder| {
            builder
                .allowlist_item("aws_crt.+http.*")
                .allowlist_item("aws_http.*")
                .allowlist_item("aws_websocket.*")
                .allowlist_item("proxy_env_var_settings")
        })
        .build()
}
