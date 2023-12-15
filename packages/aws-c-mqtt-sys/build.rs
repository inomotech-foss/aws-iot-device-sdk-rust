fn main() {
    aws_c_builder::Config::new("aws-c-mqtt")
        .aws_dependencies(&["AWS_C_HTTP"])
        .include_dir_names(&["mqtt", "mqtt/v5"])
        .build()
}
