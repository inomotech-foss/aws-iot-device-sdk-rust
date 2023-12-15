fn main() {
    let mut aws_deps = vec!["AWS_C_CAL", "AWS_C_COMMON"];
    if aws_c_builder::is_linux_like() {
        aws_deps.push("S2N");
    }
    aws_c_builder::Config::new("aws-c-io")
        .aws_dependencies(aws_deps)
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/io/.+"))
        .build()
}
