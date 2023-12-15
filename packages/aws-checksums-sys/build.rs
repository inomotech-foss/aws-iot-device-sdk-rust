fn main() {
    aws_c_builder::Config::new("aws-checksums")
        .aws_dependencies(&["AWS_C_COMMON"])
        .include_dir_names(&["checksums"])
        .build()
}
