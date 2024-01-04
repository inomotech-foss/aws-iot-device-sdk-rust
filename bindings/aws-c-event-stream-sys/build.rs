fn main() {
    aws_c_builder::Builder::new("aws-c-event-stream")
        .dependencies(["AWS_CHECKSUMS", "AWS_C_COMMON", "AWS_C_IO"])
        .build();
}
