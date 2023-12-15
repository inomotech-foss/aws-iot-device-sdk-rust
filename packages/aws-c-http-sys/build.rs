fn main() {
    aws_c_builder::Config::new("aws-c-http")
        .aws_dependencies(&["AWS_C_COMPRESSION", "AWS_C_IO"])
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/http/.+"))
        .build()
}
