fn main() {
    aws_c_builder::build("aws-c-http", &["AWS_C_COMPRESSION", "AWS_C_IO"]);
}
