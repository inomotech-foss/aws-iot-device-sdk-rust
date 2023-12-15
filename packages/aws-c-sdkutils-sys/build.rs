fn main() {
    aws_c_builder::Config::new("aws-c-sdkutils")
        .aws_dependencies(&["AWS_C_COMMON"])
        .include_dir_names(&["sdkutils"])
        .build()
}
