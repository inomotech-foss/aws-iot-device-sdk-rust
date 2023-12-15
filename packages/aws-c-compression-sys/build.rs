fn main() {
    aws_c_builder::Config::new("aws-c-compression")
        .aws_dependencies(&["AWS_C_COMMON"])
        .include_dir_names(&["compression"])
        .build()
}
