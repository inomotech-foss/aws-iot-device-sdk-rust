fn main() {
    aws_c_builder::Config::new("aws-c-auth")
        .aws_dependencies(&["AWS_C_COMMON", "AWS_C_HTTP", "AWS_C_SDKUTILS"])
        .bindgen_callback(|builder| {
            builder
                .allowlist_item("aws_auth.*")
                .allowlist_item("aws_cognito.*")
                .allowlist_item("aws_credentials.*")
                .allowlist_item("aws_imds.*")
                .allowlist_item("aws_sign.*")
                .allowlist_type("aws_should_sign_header_fn")
                .allowlist_type("aws.+credentials.+_fn")
        })
        .build()
}
