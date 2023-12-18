fn main() {
    aws_c_builder::Config::new("aws-c-compression")
        .aws_dependencies(&["AWS_C_COMMON"])
        .bindgen_callback(|builder| {
            builder
                .allowlist_item("aws_compression.*")
                .allowlist_item("aws_huffman.*")
        })
        .build()
}
