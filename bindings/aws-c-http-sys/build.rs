fn main() {
    aws_c_builder2::Builder::new("aws-c-http")
        .dependencies(["AWS_C_CAL", "AWS_C_COMMON", "AWS_C_COMPRESSION", "AWS_C_IO"])
        .build();
}
