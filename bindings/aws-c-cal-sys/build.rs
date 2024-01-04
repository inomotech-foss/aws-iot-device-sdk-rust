fn main() {
    let target = Target::determine();
    let mut builder = aws_c_builder2::Builder::new("aws-c-cal");
    match target {
        Target::Darwin => {
            builder.source_subdir("darwin");
        }
        Target::Unix => {
            builder.dependency("AWS_LC_0_12_1");
            builder.source_subdir("unix");
        }
        Target::Windows => {
            builder.source_subdir("windows");
        }
    }

    // TODO: separate bindings for ios because of
    // aws_ecc_key_pair_new_generate_random
    builder.dependency("AWS_C_COMMON").build();
}

enum Target {
    Darwin,
    Unix,
    Windows,
}

impl Target {
    fn determine() -> Self {
        let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
        let target_vendor = std::env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
        match (target_family.as_str(), target_vendor.as_str()) {
            ("windows", _) => Self::Windows,
            (_, "apple") => Self::Darwin,
            _ => Self::Unix,
        }
    }
}
