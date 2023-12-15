fn main() {
    aws_c_builder::Config::new("aws-c-event-stream")
        .aws_dependencies(&["AWS_C_COMMON", "AWS_C_IO", "AWS_CHECKSUMS"])
        .include_dir_names(&["event-stream"])
        .build()
}
