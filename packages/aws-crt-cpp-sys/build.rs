fn main() {
    aws_c_builder::Config::new("aws-crt-cpp")
        .aws_dependencies(&[
            "AWS_C_COMMON",
            "AWS_C_HTTP",
            "AWS_C_MQTT",
            "AWS_C_CAL",
            "AWS_C_AUTH",
            "AWS_C_IO",
            "AWS_CHECKSUMS",
            "AWS_C_EVENT_STREAM",
            "AWS_C_S3",
        ])
        .run_bindgen(false)
        .build()
}
