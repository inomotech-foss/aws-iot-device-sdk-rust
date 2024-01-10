#[derive(Debug)]
struct Simd {
    have_avx2_intrinsics: bool,
    have_mm256_extract_epi64: bool,
}
