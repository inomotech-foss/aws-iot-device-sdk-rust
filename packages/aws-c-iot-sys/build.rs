fn main() {
    aws_c_builder::Config::new("aws-c-iot")
        .aws_dependencies(&["AWS_C_MQTT"])
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/iotdevice/.+"))
        .build()
}
