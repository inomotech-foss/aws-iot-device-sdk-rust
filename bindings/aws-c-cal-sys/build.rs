fn main() {
    let target = Target::determine();
    eprintln!("target: {target:?}");

    let mut builder = aws_c_builder::Builder::new("aws-c-cal");
    match target {
        Target::Darwin => {
            builder.source_path("darwin");
        }
        Target::Unix => {
            builder.dependency("AWS_LC_0_12_1");
            builder.source_path("unix");
        }
        Target::Windows => {
            builder.source_path("windows");
        }
    }

    // TODO: separate bindings for ios because of
    // aws_ecc_key_pair_new_generate_random
    builder.dependency("AWS_C_COMMON").build();
}

#[derive(Debug)]
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
