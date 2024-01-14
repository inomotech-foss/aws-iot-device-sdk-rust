const USE_CPU_EXTENSIONS: bool = false;

fn main() {
    let ctx = aws_c_builder::Context::new();
    let mut builder = ctx.builder("aws-checksums");

    if USE_CPU_EXTENSIONS {
        if ctx.is_aws_arch_intel() {
            if ctx.aws_have_gcc_inline_asm() {
                builder.source_path("intel/asm");
            } else if ctx.is_msvc() {
                builder.source_path("intel/visualc");
            }
        } else if ctx.is_msvc() && ctx.is_aws_arch_arm64() {
            builder.source_path("arm");
        } else if ctx.is_aws_arch_arm64() {
            // TODO: custom file properties
            builder.source_path("arm");
        } else if !ctx.is_msvc() && ctx.is_aws_arch_arm32() {
            // TODO
        } else {
            builder.source_path("generic");
        }
    } else {
        builder.source_path("generic");
    }

    builder
        .aws_set_common_properties()
        .dependencies(["aws-c-common"])
        .build();
}
