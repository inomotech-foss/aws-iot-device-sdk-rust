use std::path::Path;

fn main() {
    aws_c_builder2::Builder::new(Path::new("aws-c-event-stream"))
        .dependencies(["AWS_C_CHECKSUMS", "AWS_C_COMMON", "AWS_C_IO"])
        .build();
}
