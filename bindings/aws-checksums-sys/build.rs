fn main() {
    // TODO: CPU-specific extensions
    aws_c_builder::Builder::new("aws-checksums")
        .dependency("AWS_C_COMMON")
        .source_path("generic")
        .build();
}
