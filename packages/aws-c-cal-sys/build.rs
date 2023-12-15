fn main() {
    let mut aws_deps = vec!["AWS_C_COMMON"];
    if aws_c_builder::is_linux_like() {
        aws_deps.push("AWS_LC");
    }

    aws_c_builder::Config::new("aws-c-cal")
        .aws_dependencies(aws_deps)
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/cal/.+"))
        .build()
}
