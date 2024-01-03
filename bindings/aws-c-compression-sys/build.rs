use std::path::Path;

fn main() {
    aws_c_builder2::Builder::new(Path::new("aws-c-compression"))
        .dependency("AWS_C_COMMON")
        .build();
}
