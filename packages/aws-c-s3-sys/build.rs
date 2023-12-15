fn main() {
    aws_c_builder::Config::new("aws-c-s3")
        .aws_dependencies(&["AWS_C_COMMON", "AWS_C_AUTH", "AWS_CHECKSUMS"])
        .include_dir_names(&["s3"])
        .build()
}
