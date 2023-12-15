fn main() {
    aws_c_builder::Config::new("aws-c-mqtt")
        .aws_dependencies(&["AWS_C_HTTP"])
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/mqtt/.+"))
        .build()
}
