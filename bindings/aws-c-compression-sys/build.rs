fn main() {
    aws_c_builder::Builder::new("aws-c-compression")
        .dependency("AWS_C_COMMON")
        .build();
}
