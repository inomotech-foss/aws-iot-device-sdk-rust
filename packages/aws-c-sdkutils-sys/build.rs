fn main() {
    aws_c_builder::Config::new("aws-c-sdkutils")
        .aws_dependencies(&["AWS_C_COMMON"])
        .bindgen_callback(|builder| {
            builder
                .allowlist_item("aws_endpoints.*")
                .allowlist_item("aws_partitions.*")
                .allowlist_item("aws_profile.*")
                .allowlist_item("aws_resource_name.*")
                .allowlist_item("aws_sdkutils.*")
        })
        .build()
}
