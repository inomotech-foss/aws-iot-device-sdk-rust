fn main() {
    aws_c_builder::Builder::new("aws-c-sdkutils")
        .dependency("AWS_C_COMMON")
        .build();
}
