fn main() {
    aws_c_builder::Config::new("aws-c-iot")
        .aws_dependencies(&["AWS_C_MQTT"])
        .include_dir_names(&["iotdevice"])
        .build()
}
