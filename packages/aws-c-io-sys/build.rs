fn main() {
    aws_c_builder::Config::new("aws-c-io")
        .aws_dependencies(&["AWS_C_CAL", "AWS_C_COMMON"])
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/io/.+"))
        .build()
}
