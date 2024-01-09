fn main() {
    aws_c_builder::Builder::new("aws-c-mqtt")
        .dependencies(["AWS_C_COMMON", "AWS_C_HTTP", "AWS_C_IO"])
        .source_path("v5")
        .build();
}
