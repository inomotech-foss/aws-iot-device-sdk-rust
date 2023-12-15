fn main() {
    aws_c_builder::Config::new("aws-c-auth")
        .aws_dependencies(&["AWS_C_COMMON", "AWS_C_SDKUTILS", "AWS_C_HTTP"])
        .include_dir_names(&["auth"])
        .bindgen_callback(|config| config.blocklist_item("aws_client_bootstrap"))
        .build()
}
