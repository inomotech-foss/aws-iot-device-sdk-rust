fn main() {
    aws_c_builder2::Builder::new("aws-c-auth")
        .dependencies([
            "AWS_C_CAL",
            "AWS_C_COMMON",
            "AWS_C_HTTP",
            "AWS_C_IO",
            "AWS_C_SDKUTILS",
        ])
        .build();
}
