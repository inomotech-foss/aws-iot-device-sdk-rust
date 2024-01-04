fn main() {
    aws_c_builder::Builder::new("aws-c-iot")
        .dependencies(["AWS_C_COMMON", "AWS_C_HTTP", "AWS_C_IO", "AWS_C_MQTT"])
        .build();
}
