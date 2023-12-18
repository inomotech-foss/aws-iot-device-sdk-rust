fn main() {
    aws_c_builder::Config::new("aws-c-mqtt")
        .aws_dependencies(&["AWS_C_HTTP"])
        .bindgen_callback(|builder| {
            builder
                .allowlist_item("aws_mqtt.*")
                .allowlist_item("on_connection_closed_data")
        })
        .build()
}
