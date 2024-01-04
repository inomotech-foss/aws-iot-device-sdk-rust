fn main() {
    // TODO: CPU-specific extensions
    aws_c_builder2::Builder::new("aws-checksums")
        .dependency("AWS_C_COMMON")
        .source_subdir("generic")
        .build();
}
