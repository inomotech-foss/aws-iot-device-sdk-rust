fn main() {
    aws_c_builder::Config::new("aws-c-cal")
        .aws_dependencies(&["AWS_C_COMMON"])
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/cal/.+"))
        .build()
}
