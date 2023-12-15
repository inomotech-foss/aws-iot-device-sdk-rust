fn main() {
    aws_c_builder::Config::new("aws-lc")
        .bindgen_callback(|builder| builder.allowlist_file(".+/aws/lc/.+"))
        .build()
}
